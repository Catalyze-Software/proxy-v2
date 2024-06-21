use crate::{helpers::guards::is_developer, logic::id_logic::IDLogic};
use ic_cdk::query;

#[query(guard = "is_developer")]
fn _dev_get_all_ids() -> Vec<(String, u64)> {
    IDLogic::get_all()
}
