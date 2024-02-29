use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    paged_response::PagedResponse,
    report::{PostReport, Report, ReportFilter, ReportResponse, ReportSort},
};
use ic_cdk::caller;
use std::collections::HashMap;

use crate::storage::{MemberStore, ReportStore, StorageMethods};

pub struct ReportCalls;

impl ReportCalls {
    pub fn add_report(post_report: PostReport) -> Result<ReportResponse, ApiError> {
        let (_, member) = MemberStore::get(caller())?;

        if !member.is_group_joined(&post_report.group_id) {
            return Err(ApiError::bad_request());
        }

        let report = Report::new(post_report);
        let result = ReportStore::insert(report);
        ReportResponse::from_result(result)
    }

    pub fn get_report(report_id: u64) -> Result<ReportResponse, ApiError> {
        ReportResponse::from_result(ReportStore::get(report_id))
    }

    pub fn get_reports(
        limit: usize,
        page: usize,
        sort: ReportSort,
        filters: Vec<FilterType<ReportFilter>>,
        group_id: u64,
    ) -> Result<PagedResponse<ReportResponse>, ApiError> {
        let reports =
            ReportStore::filter(|_, report| report.group_id.is_some_and(|id| id == group_id));

        // split the filters into or and and filters
        let mut or_filters: Vec<ReportFilter> = vec![];
        let mut and_filters: Vec<ReportFilter> = vec![];
        for filter_type in filters {
            use FilterType::*;
            match filter_type {
                And(filter_value) => and_filters.push(filter_value),
                Or(filter_value) => or_filters.push(filter_value),
            }
        }

        // filter the groups based on the `OR` filters
        let mut or_filtered_reports: HashMap<u64, Report> = HashMap::new();
        for filter in or_filters {
            for (id, report) in &reports {
                if filter.is_match(id, report) {
                    or_filtered_reports.insert(id.clone(), report.clone());
                }
            }
        }

        // filter the `or_filtered` groups based on the `AND` filters
        let mut and_filtered_reports: HashMap<u64, Report> = HashMap::new();
        for filter in and_filters {
            for (id, report) in &or_filtered_reports {
                if filter.is_match(id, report) {
                    and_filtered_reports.insert(id.clone(), report.clone());
                }
            }
        }

        let sorted_reports = sort.sort(and_filtered_reports);
        let result: Vec<ReportResponse> = sorted_reports
            .into_iter()
            .map(|data| ReportResponse::new(data.0, data.1))
            .collect();

        Ok(PagedResponse::new(page, limit, result))
    }
}
