use super::ledger_logic::Ledger;
use crate::{storage::boosts, E8S_PER_DAY_BOOST_COST};
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    boosted::{Boost, BoostedEntry, BoostedFilter},
    subject::{Subject, SubjectType},
    CanisterResult, Filter, StorageClient, StorageClientInsertable,
};
use ic_cdk::{api::time, caller};
use ic_ledger_types::Tokens;
use std::{cell::RefCell, time::Duration};

thread_local! {
    pub static LAST_BLOCK_HEIGHT: RefCell<u64> = RefCell::new(u64::default());
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
        let boost = boosts()
            .find(BoostedFilter::Subject(subject.clone()).to_vec())
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
        boosts().insert(boost).await?;
        Ok(seconds)
    }

    async fn update_existing_boost(
        boost_id: u64,
        mut boost: Boost,
        seconds: u64,
    ) -> CanisterResult<u64> {
        // Update the boost with the purchased seconds
        let remaining_seconds = Self::get_seconds_left_for_boost(boost_id).await?;
        let new_seconds = remaining_seconds + seconds;

        boost.seconds = new_seconds;
        boost.updated_at = time();

        boosts().update(boost_id, boost).await?;

        Ok(new_seconds)
    }

    pub async fn remove_boost(boost_id: u64) -> CanisterResult<()> {
        boosts().remove(boost_id).await?;
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

    pub async fn get_seconds_left_for_boost(boost_id: u64) -> CanisterResult<u64> {
        let (_, boosted) = boosts().get(boost_id).await?;
        let time_left: u64 = Duration::from_nanos(boosted.updated_at).as_secs() + boosted.seconds;
        Ok(time_left - Duration::from_nanos(time()).as_secs())
    }

    pub async fn get_boost_by_subject(subject: Subject) -> CanisterResult<Option<BoostedEntry>> {
        boosts()
            .find(BoostedFilter::Subject(subject).to_vec())
            .await
    }

    pub async fn get_boosts_by_subject(subject: SubjectType) -> CanisterResult<Vec<BoostedEntry>> {
        boosts()
            .filter(BoostedFilter::SubjectType(subject).to_vec())
            .await
    }
}
