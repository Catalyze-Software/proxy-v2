#![allow(unused)]
use crate::{ENV, SENDER};
use candid::Principal;
use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    paged_response::PagedResponse,
    report::{PostReport, ReportFilter, ReportResponse},
};
use pocket_ic::{query_candid_as, update_candid_as};

pub fn add_report(
    value: PostReport,
    group_identifier: Principal,
    member_identifier: Principal,
) -> ReportResponse {
    update_candid_as::<(PostReport, Principal, Principal), (Result<ReportResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "add_report",
        (value, group_identifier, member_identifier),
    )
    .expect("Failed to call add_report from pocket ic")
    .0
    .expect("Failed to call add_report")
}

pub fn get_report(
    identifier: Principal,
    group_identifier: Principal,
    member_identifier: Principal,
) -> ReportResponse {
    query_candid_as::<(Principal, Principal, Principal), (Result<ReportResponse, ApiError>,)>(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_report",
        (identifier, group_identifier, member_identifier),
    )
    .expect("Failed to call get_report from pocket ic")
    .0
    .expect("Failed to call get_report")
}

pub fn get_reports(
    limit: usize,
    page: usize,
    sort: String,
    filter_type: Vec<FilterType<ReportFilter>>,
    group_identifier: Principal,
    member_identifier: Principal,
) -> PagedResponse<ReportResponse> {
    query_candid_as::<
        (
            usize,
            usize,
            String,
            Vec<FilterType<ReportFilter>>,
            Principal,
            Principal,
        ),
        (Result<PagedResponse<ReportResponse>, ApiError>,),
    >(
        &ENV.pic,
        ENV.canister_id,
        SENDER.with(|s| s.borrow().unwrap()),
        "get_reports",
        (
            limit,
            page,
            sort,
            filter_type,
            group_identifier,
            member_identifier,
        ),
    )
    .expect("Failed to call get_reports from pocket ic")
    .0
    .expect("Failed to call get_reports")
}
