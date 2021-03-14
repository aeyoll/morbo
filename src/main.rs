use tide::prelude::*;
use tide::Request;

#[macro_use]
extern crate clap;
use clap::App;

#[derive(Debug, Deserialize)]
struct CspReport {
    #[serde(alias = "csp-report")]
    pub csp_report: CspReportContent,
}

#[derive(Debug, Serialize, Deserialize)]
struct CspReportContent {
    /// The URI of the resource that was blocked from loading by the
    /// Content Security Policy. If the blocked URI is from a different
    /// origin than the document-uri, then the blocked URI is truncated
    /// to contain just the scheme, host, and port.
    #[serde(alias = "blocked-uri")]
    blocked_uri: String,

    /// Either "enforce" or "report" depending on whether the
    /// Content-Security-Policy-Report-Only header or
    /// the Content-Security-Policy header is used.
    disposition: String,

    /// The URI of the document in which the violation occurred.
    #[serde(alias = "document-uri")]
    document_uri: String,

    /// The directive whose enforcement caused the violation. Some browsers
    /// may provide different values, such as Chrome providing
    /// style-src-elem/style-src-attr, even when the actual enforced directive
    /// was style-src.
    #[serde(alias = "effective-directive")]
    effective_directive: String,

    /// The original policy as specified by the Content-Security-Policy
    /// HTTP header.
    #[serde(alias = "original-policy")]
    original_policy: String,

    /// The referrer of the document in which the violation occurred.
    referrer: String,

    /// The first 40 characters of the inline script, event handler, or
    /// style that caused the violation. Only applicable to script-src*
    /// and style-src* violations, when they contain the 'report-sample'
    #[serde(alias = "script-sample")]
    script_sample: String,

    /// The HTTP status code of the resource on which the global object
    /// was instantiated.
    #[serde(alias = "status-code")]
    status_code: String,

    /// The name of the policy section that was violated.
    #[serde(alias = "violated-directive")]
    violated_directive: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let binding = format!("127.0.0.1:{}", port);
    println!("Launching server on {}", binding);

    let mut app = tide::new();
    app.at("/_/csp-reports").post(csp_report);
    app.listen(&binding).await?;
    Ok(())
}

async fn csp_report(mut req: Request<()>) -> tide::Result {
    let CspReport { csp_report } = req.body_json().await?;
    Ok(format!("CSP report: {}", serde_json::to_string_pretty(&csp_report).unwrap()).into())
}
