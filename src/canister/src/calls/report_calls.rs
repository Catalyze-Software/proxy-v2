/// # Report methods
/// # TODO:
/// * Check if the guard are correctly placed

/// # Questions
/// Check the public / private access of these calls? (anon / registered / group role)
///
use candid::Principal;
use ic_cdk::{query, update};

use crate::{
    helpers::{group_permission::can_write, guards::has_access},
    logic::report_logic::ReportCalls,
};
use canister_types::models::{
    api_error::ApiError,
    filter_type::FilterType,
    identifier::Identifier,
    paged_response::PagedResponse,
    permission::PermissionType,
    report::{PostReport, ReportFilter, ReportResponse, ReportSort},
};

/// Add a report
/// # Arguments
/// * `post_report` - The report to add
/// # Returns
/// * `ReportResponse` - The added report
/// # Errors
/// * `ApiError` - If something went wrong while adding the profile
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[update(guard = "has_access")]
pub fn add_report(post_report: PostReport) -> Result<ReportResponse, ApiError> {
    ReportCalls::add_report(post_report)
}

/// Get a report
/// # Arguments
/// * `identifier` - The identifier of the report
/// * `group_identifier` - Used to check if the user has access to the group
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `ReportResponse` - The report
/// # Errors
/// * `ApiError` - If something went wrong while getting the report
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_report(
    identifier: Principal,
    group_identifier: Principal,
) -> Result<ReportResponse, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_write(group_id, PermissionType::Group(None))?;
    ReportCalls::get_report(group_id)
}

/// Get reports
/// # Arguments
/// * `limit` - The maximum number of reports to return per page
/// * `page` - The page number
/// * `sort` - The sort direction
/// * `filters` - The filters to apply
/// * `filter_type` - The filter type
/// * `group_identifier` - Used to check if the user has access to the group
/// * `member_identifier` - Used to check if the user has the correct group roles
/// # Returns
/// * `PagedResponse<ReportResponse>` - The reports in a paged response
/// # Errors
/// * `ApiError` - If something went wrong while getting the reports
/// # Note
/// This function is guarded by the [`has_access`](has_access) function.
#[query(guard = "has_access")]
pub fn get_reports(
    limit: usize,
    page: usize,
    sort: ReportSort,
    filters: Vec<FilterType<ReportFilter>>,
    group_identifier: Principal,
) -> Result<PagedResponse<ReportResponse>, ApiError> {
    let group_id = Identifier::from(group_identifier).id();
    can_write(group_id, PermissionType::Group(None))?;
    ReportCalls::get_reports(limit, page, sort, filters, group_id)
}
