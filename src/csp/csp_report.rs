use crate::csp::csp_report_content::CspReportContent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CspReport {
    #[serde(alias = "csp-report")]
    pub csp_report: CspReportContent,
}
