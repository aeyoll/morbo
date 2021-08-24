use tide::prelude::*;

use crate::mail_configuration::MailConfiguration;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::Error as LettreError;
use lettre::smtp::response::Response as LettreResponse;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct CspReportContent {
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
    pub fn send_email(
        &self,
        mail_configuration: &MailConfiguration,
    ) -> Result<LettreResponse, LettreError> {
        let email = EmailBuilder::new()
            .from(("csr@example.org", "CSP Report"))
            .to((&mail_configuration.to_email, &mail_configuration.to_name))
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
