use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    page: usize,
    limit: usize,
    total: usize,
    number_of_pages: usize,
    data: Vec<T>,
}

impl<T: Clone> PagedResponse<T> {
    pub fn new(mut page: usize, mut limit: usize, data: Vec<T>) -> Self {
        let total = data.len();

        if page == 0 {
            return Self {
                page,
                limit,
                total,
                number_of_pages: 0,
                data: vec![],
            };
        }

        if limit >= total {
            limit = total;
        }

        let _number_of_pages_float = total as f32 / limit as f32;
        let number_of_pages = _number_of_pages_float.ceil() as usize;

        let mut start_limit = (page - 1) * limit;
        let mut end_limit = (page - 1) * limit + limit;

        if page >= number_of_pages {
            page = number_of_pages;
            start_limit = number_of_pages * limit - limit;
            end_limit = total;
        }

        Self {
            page,
            limit,
            total,
            number_of_pages,
            data: data[start_limit..end_limit].to_vec(),
        }
    }
}
