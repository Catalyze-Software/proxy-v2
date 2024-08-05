use catalyze_shared::{
    api_error::ApiError,
    paged_response::PagedResponse,
    report::{PostReport, Report, ReportFilter, ReportResponse, ReportSort},
    CanisterResult,
};
use ic_cdk::caller;
use std::collections::HashMap;

use crate::storage::{MemberStore, ReportStore, StorageInsertable, StorageQueryable};

use super::profile_logic::ProfileCalls;

pub struct ReportCalls;

impl ReportCalls {
    pub async fn add_report(post_report: PostReport) -> CanisterResult<ReportResponse> {
        let (_, member) = MemberStore::get(caller())?;

        if !member.is_group_joined(&post_report.group_id) {
            return Err(ApiError::bad_request());
        }

        let report = Report::new(post_report);
        let result = ReportStore::insert(report)?;
        ReportResponse::from_result(
            Ok(result.clone()),
            ProfileCalls::get_subject_response_by_subject(&result.1.subject).await,
        )
    }

    pub async fn get_report(report_id: u64) -> CanisterResult<ReportResponse> {
        let result = ReportStore::get(report_id)?;
        ReportResponse::from_result(
            Ok(result.clone()),
            ProfileCalls::get_subject_response_by_subject(&result.1.subject).await,
        )
    }

    pub async fn get_reports(
        limit: usize,
        page: usize,
        sort: ReportSort,
        filters: Vec<ReportFilter>,
        group_id: u64,
    ) -> CanisterResult<PagedResponse<ReportResponse>> {
        let mut reports =
            ReportStore::filter(|_, report| report.group_id.is_some_and(|id| id == group_id))
                .into_iter()
                .collect::<HashMap<u64, Report>>();

        for filter in filters {
            for (id, report) in &reports.clone() {
                if !filter.is_match(id, report) {
                    reports.remove(id);
                }
            }
        }

        let sorted_reports = sort.sort(reports);

        let mut result = vec![];

        for data in sorted_reports.into_iter() {
            result.push(ReportResponse::new(
                data.0,
                data.1.clone(),
                ProfileCalls::get_subject_response_by_subject(&data.1.subject).await,
            ));
        }

        Ok(PagedResponse::new(page, limit, result))
    }
}
