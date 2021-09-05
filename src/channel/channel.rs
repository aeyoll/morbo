use crate::csp::csp_report_content::CspReportContent;
use anyhow::Error;

pub trait Channel<T> {
    /// Construct a transport from environment variables
    fn load_from_env() -> Self;

    /// Implements how the transport is sending the report
    fn send_report(&self, report: &CspReportContent) -> Result<T, Error>;
}