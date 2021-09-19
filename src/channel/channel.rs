use crate::csp::csp_report_content::CspReportContent;
use anyhow::Error;

/// A channel represent's how a CSP report is sent
pub trait Channel<T> {
    /// Construct a channel from environment variables
    fn load_from_env() -> Self;

    /// Implements how the channel is sending the report
    fn send_report(&self, report: &CspReportContent) -> Result<T, Error>;
}
