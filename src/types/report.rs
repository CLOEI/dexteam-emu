use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReportResponse {
    pub data: OutletData,
}

#[derive(Debug, Deserialize)]
pub struct OutletData {
    pub outlet: Outlet,
}

#[derive(Debug, Deserialize)]
pub struct Outlet {
    pub name: String,
    pub employee: Employee,
}

#[derive(Debug, Deserialize)]
pub struct Employee {
    pub name: String,
    pub attendance: Attendance,
}

#[derive(Debug, Deserialize)]
pub struct Attendance {
    pub rule: Rule,
    pub summary_attendance: SummaryAttendance,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub image: bool,
}

#[derive(Debug, Deserialize)]
pub struct SummaryAttendance {
    pub total_present: u32,
    pub total_absent: u32,
    pub total_late: u32,
    pub total_leave: u32,
    pub report: Vec<Report>,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    pub date: String,
    #[serde(rename = "Shift")]
    pub shift: Vec<Shift>,
}

#[derive(Debug, Deserialize)]
pub struct Shift {
    pub clock_time_id: u64,
    pub early_check_out: Option<String>,
    #[serde(rename = "in")]
    pub check_in: Option<String>,
    #[serde(rename = "out")]
    pub check_out: Option<String>,
    pub is_late: u32,
    pub is_present: u32,
    pub shift_in: String,
    pub shift_out: String,
    pub is_absent: u32,
    pub is_leave: u32,
    pub working_duration: Option<String>,
    pub working_hours_id: u64,
    pub working_hours_detail: WorkingHoursDetail,
    pub breaks: Vec<Break>,
    pub is_holiday: u32,
    pub holiday: Option<String>,
    pub leave_history: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WorkingHoursDetail {
    pub working_hours_id: u64,
    pub name: String,
    pub timezone_details: TimezoneDetails,
}

#[derive(Debug, Deserialize)]
pub struct TimezoneDetails {
    pub id: u64,
    pub name: String,
    pub utc_offset: String,
    pub utc_dst_offset: String,
}

#[derive(Debug, Deserialize)]
pub struct Break {
    pub break_time_id: u64,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub is_check_in_needed: bool,
    pub working_hours_daily_history_id: u64,
    pub break_attendance_id: u64,
    pub break_in: String,
    pub break_out: String,
    pub attendance_id: u64,
}