use tide::prelude::*;
use tide::Request;

#[macro_use]
extern crate clap;
use clap::App;

extern crate lettre;
extern crate lettre_email;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::response::Response as LettreResponse;
use lettre::smtp::error::Error as LettreError;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

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
    disposition: Option<String>,

    /// The URI of the document in which the violation occurred.
    #[serde(alias = "document-uri")]
    document_uri: String,

    /// The directive whose enforcement caused the violation. Some browsers
    /// may provide different values, such as Chrome providing
    /// style-src-elem/style-src-attr, even when the actual enforced directive
    /// was style-src.
    #[serde(alias = "effective-directive")]
    effective_directive: Option<String>,

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
    script_sample: Option<String>,

    /// The HTTP status code of the resource on which the global object
    /// was instantiated.
    #[serde(alias = "status-code")]
    status_code: Option<String>,

    /// The name of the policy section that was violated.
    #[serde(alias = "violated-directive")]
    violated_directive: String,
}

impl CspReportContent {
    fn send_email(&self, mail_configuration: MailConfiguration) -> Result<LettreResponse, LettreError> {
        let email = EmailBuilder::new()
            .from(("csr@example.org", "CSP Report"))
            .to((
                &mail_configuration.to_email,
                &mail_configuration.to_name,
            ))
            .subject("[CSP] New report")
            .body(format!(
                "New report {}",
                serde_json::to_string_pretty(self).unwrap()
            ))
            .build()
            .unwrap();

        let credentials = Credentials::new(
            mail_configuration.smtp_username.to_owned(),
            mail_configuration.smtp_password.to_owned(),
        );
        let mut mailer = SmtpClient::new_simple(&mail_configuration.smtp_server)
            .unwrap()
            // Add credentials for authentication
            .credentials(credentials)
            // Configure expected authentication mechanism
            .authentication_mechanism(Mechanism::Plain)
            .transport();

        // Send the email
        mailer.send(email.into())
    }
}

#[derive(Debug, Default)]
pub struct MailConfiguration {
    to_name: String,
    to_email: String,
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
}

impl MailConfiguration {
    pub fn new(
        to_name: String,
        to_email: String,
        smtp_server: String,
        smtp_username: String,
        smtp_password: String,
    ) -> Self {
        MailConfiguration {
            to_name,
            to_email,
            smtp_server,
            smtp_username,
            smtp_password,
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load env variables
    dotenv().ok();

    // Cli args
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let binding = format!("127.0.0.1:{}", port);
    println!("Launching server on {}", binding);

    let mut app = tide::new();
    app.at("/_/csp-reports").post(csp_report_action);
    app.listen(&binding).await?;
    Ok(())
}

async fn csp_report_action(mut req: Request<()>) -> tide::Result {
    let CspReport { csp_report } = req.body_json().await?;

    let mail_configuration = MailConfiguration::new(
        env::var("TO_NAME").unwrap(),
        env::var("TO_EMAIL").unwrap(),
        env::var("SMTP_SERVER").unwrap(),
        env::var("SMTP_USERNAME").unwrap(),
        env::var("SMTP_PASSWORD").unwrap(),
    );

    csp_report.send_email(mail_configuration).unwrap();

    Ok(format!("CSP report: {}", serde_json::to_string_pretty(&csp_report).unwrap()).into())
}
