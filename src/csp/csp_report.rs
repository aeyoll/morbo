use crate::csp::csp_report_content::CspReportContent;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
pub struct CspReport {
    #[serde(alias = "csp-report")]
    pub csp_report: CspReportContent,
}
