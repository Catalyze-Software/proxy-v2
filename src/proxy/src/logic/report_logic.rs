use catalyze_shared::{
    api_error::ApiError,
    paged_response::PagedResponse,
    report::{PostReport, Report, ReportFilter, ReportResponse, ReportSort},
    CanisterResult, StorageClient, StorageClientInsertable,
};
use ic_cdk::caller;

use crate::storage::{groups, reports};

use super::profile_logic::ProfileCalls;

pub struct ReportCalls;

impl ReportCalls {
    pub async fn add_report(post_report: PostReport) -> CanisterResult<ReportResponse> {
        let (_, group) = groups().get(post_report.group_id).await?;

        let is_group_joined = group
            .get_members()
            .into_iter()
            .any(|member| member == caller());

        if is_group_joined {
            return Err(ApiError::bad_request());
        }

        let report = Report::new(post_report);
        let result = reports().insert(report).await?;
        ReportResponse::from_result(
            Ok(result.clone()),
            ProfileCalls::get_subject_response_by_subject(&result.1.subject).await,
        )
    }

    pub async fn get_report(report_id: u64) -> CanisterResult<ReportResponse> {
        let result = reports().get(report_id).await?;
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
        let filters = vec![ReportFilter::GroupId(group_id)]
            .into_iter()
            .chain(filters)
            .collect();

        let resp = reports()
            .filter_paginated(limit, page, sort, filters)
            .await?;

        let mut result = vec![];

        for data in resp.data.into_iter() {
            result.push(ReportResponse::new(
                data.0,
                data.1.clone(),
                ProfileCalls::get_subject_response_by_subject(&data.1.subject).await,
            ));
        }

        PagedResponse::new(page, limit, result).into_result()
    }
}
