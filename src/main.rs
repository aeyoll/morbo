use std::fmt;
use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct CspReport {
    #[serde(alias = "csp-report")]
    pub csp_report: CspReportContent,
}

#[derive(Debug, Deserialize)]
struct CspReportContent {
    #[serde(alias = "document-uri")]
    document_uri: String,

    referrer: String,

    #[serde(alias = "blocked-uri")]
    blocked_uri: String,

    /// The name of the policy section that was violated.
    #[serde(alias = "violated-directive")]
    violated_directive: String,

    #[serde(alias = "original-policy")]
    original_policy: String,
}

impl fmt::Display for CspReportContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "document_uri: \"{}\", referrer: \"{}\", blocked_uri: \"{}\", violated_directive: \"{}\", original_policy: \"{}\"",
            self.document_uri, self.referrer, self.blocked_uri, self.violated_directive, self.original_policy
        )
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/_/csp-reports").post(csp_report);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn csp_report(mut req: Request<()>) -> tide::Result {
    let CspReport { csp_report } = req.body_json().await?;
    Ok(format!("CSP report: {}", csp_report).into())
}