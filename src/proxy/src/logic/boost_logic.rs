use super::ledger_logic::Ledger;
use crate::{storage::boosteds, E8S_PER_DAY_BOOST_COST};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    boosted::{Boost, BoostedEntry, BoostedFilter},
    subject::{Subject, SubjectType},
    CanisterResult, StorageClient, StorageClientInsertable,
};
use ic_cdk::{api::time, caller};
use ic_cdk_timers::{clear_timer, set_timer, TimerId};
use ic_ledger_types::Tokens;
use std::{cell::RefCell, collections::HashMap, time::Duration};

thread_local! {
    pub static LAST_BLOCK_HEIGHT: RefCell<u64> = RefCell::new(u64::default());
    pub static TIMERS: RefCell<HashMap<u64, TimerId>> = RefCell::new(HashMap::default());
}

pub struct BoostCalls;

impl BoostCalls {
    pub async fn boost(subject: Subject, blockheight: u64) -> CanisterResult<u64> {
        if !matches!(subject, Subject::Group(_) | Subject::Event(_)) {
            return Err(ApiError::bad_request().add_message("Invalid identifier"));
        }

        let tokens = Ledger::validate_transaction(caller(), blockheight).await?;

        if blockheight <= Self::get_last_block_height() {
            return Err(ApiError::bad_request()
                .add_message("Blockheight is lower than the last blockheight"));
        }

        Self::set_last_block_height(blockheight);

        let days = Self::calculate_days(tokens);
        let seconds = Self::get_seconds_from_days(days);
        let boost = boosteds()
            .find(BoostedFilter::Subject(subject.clone()).into())
            .await?;

        if let Some((id, boost)) = boost {
            return Self::update_existing_boost(id, boost, seconds).await;
        }

        Self::new_boost(subject, seconds, caller(), blockheight).await
    }

    async fn new_boost(
        subject: Subject,
        seconds: u64,
        owner: Principal,
        blockheight: u64,
    ) -> CanisterResult<u64> {
        let boost = Boost::new(subject, seconds, owner, blockheight);

        let (new_boost_id, _) = boosteds().insert(boost).await?;

        let timer_id = set_timer(Duration::from_secs(seconds), move || {
            ic_cdk::spawn(async move {
                let _ = Self::remove_boost(new_boost_id).await;
            });
        });

        Self::set_timer_id(new_boost_id, timer_id);

        Ok(seconds)
    }

    async fn update_existing_boost(
        boost_id: u64,
        mut boost: Boost,
        seconds: u64,
    ) -> CanisterResult<u64> {
        // Get and clear the existing timer
        if let Some(existing_timer_id) = Self::get_timer_id(boost_id) {
            clear_timer(existing_timer_id);
        }

        // Update the boost with the purchased seconds
        let remaining_seconds = Self::get_seconds_left_for_boost(boost_id).await?;
        let new_seconds = remaining_seconds + seconds;

        boost.seconds = new_seconds;
        boost.updated_at = time();

        boosteds().update(boost_id, boost.clone()).await?;

        // Remove the old timer and set a new timer with the updated seconds
        let timer_id = set_timer(Duration::from_secs(new_seconds), move || {
            ic_cdk::spawn(async move {
                let _ = boosteds().remove(boost_id).await;
            });
        });

        Self::set_timer_id(boost_id, timer_id);
        Ok(new_seconds)
    }

    pub async fn remove_boost(boost_id: u64) -> CanisterResult<()> {
        boosteds().remove(boost_id).await?;
        Self::remove_timer_id(&boost_id);
        Ok(())
    }

    pub fn calculate_days(tokens: Tokens) -> u64 {
        ((tokens.e8s() as f64) / (E8S_PER_DAY_BOOST_COST as f64)).round() as u64
    }

    pub fn get_seconds_from_days(days: u64) -> u64 {
        days * 24 * 60 * 60
    }

    pub fn set_last_block_height(block_height: u64) {
        LAST_BLOCK_HEIGHT.with(|b| {
            *b.borrow_mut() = block_height;
        });
    }

    pub fn get_last_block_height() -> u64 {
        LAST_BLOCK_HEIGHT.with(|b| *b.borrow())
    }

    pub fn set_timer_id(boost_id: u64, timer_id: TimerId) {
        TIMERS.with(|t| {
            t.borrow_mut().insert(boost_id, timer_id);
        });
    }

    pub fn get_timer_id(boost_id: u64) -> Option<TimerId> {
        TIMERS.with(|t| t.borrow().get(&boost_id).cloned())
    }

    pub fn remove_timer_id(boost_id: &u64) {
        TIMERS.with(|t| {
            t.borrow_mut().remove(boost_id);
        });
    }

    pub async fn get_seconds_left_for_boost(boost_id: u64) -> CanisterResult<u64> {
        let (_, boosted) = boosteds().get(boost_id).await?;
        let time_left: u64 = Duration::from_nanos(boosted.updated_at).as_secs() + boosted.seconds;
        Ok(time_left - Duration::from_nanos(time()).as_secs())
    }

    pub async fn get_boost_by_subject(subject: Subject) -> CanisterResult<Option<BoostedEntry>> {
        let resp = boosteds()
            .find(BoostedFilter::Subject(subject).into())
            .await?;

        Ok(resp)
    }

    pub async fn get_boosts_by_subject(subject: SubjectType) -> CanisterResult<Vec<BoostedEntry>> {
        boosteds()
            .filter(BoostedFilter::SubjectType(subject).into())
            .await
    }

    pub async fn start_timers_after_upgrade() -> CanisterResult<()> {
        let boosts = boosteds().get_all().await?;

        for (boost_id, _) in boosts {
            let seconds_left = Self::get_seconds_left_for_boost(boost_id).await?;
            let timer_id = set_timer(Duration::from_secs(seconds_left), move || {
                ic_cdk::spawn(async move {
                    let _ = Self::remove_boost(boost_id).await;
                });
            });

            Self::set_timer_id(boost_id, timer_id);
        }

        Ok(())
    }
}
