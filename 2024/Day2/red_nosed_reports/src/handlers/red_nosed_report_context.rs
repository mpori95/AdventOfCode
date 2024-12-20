#[derive(Default)]
pub struct RedNosedReportContext {
    pub is_decreasing: bool,
    pub is_increasing: bool,
    pub safe_reports: i32,
    pub first_level: Option<i32>,
}