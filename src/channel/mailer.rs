use crate::channel::channel::Channel;
use crate::csp::csp_report_content::CspReportContent;
use anyhow::Error;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::SmtpResult;
use lettre::{ClientSecurity, SmtpClient, SmtpTransport};
use lettre_email::EmailBuilder;

use lettre::Transport;
use std::env;

/// Mailer channel
pub struct Mailer {
    pub from_name: String,
    pub from_email: String,

    pub to_name: String,
    pub to_email: String,

    pub smtp_hostname: String,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
}

impl Mailer {
    pub fn get_transport(&self) -> Result<SmtpTransport, Error> {
        let addr = (
            self.smtp_hostname.as_str(),
            self.smtp_port,
        );
        let security = ClientSecurity::None;

        let mut smtp_client = SmtpClient::new(addr, security)?;

        if self.smtp_username.is_some() && self.smtp_password.is_some()
        {
            let credentials = Credentials::new(
                self
                    .smtp_username
                    .as_ref()
                    .unwrap()
                    .to_owned(),
                self
                    .smtp_password
                    .as_ref()
                    .unwrap()
                    .to_owned(),
            );

            smtp_client = smtp_client
                .credentials(credentials)
                .authentication_mechanism(Mechanism::Plain);
        }

        let transport = smtp_client.transport();
        Ok(transport)
    }
}

impl Channel<SmtpResult> for Mailer {
    fn load_from_env() -> Self {
        let from_name = env::var("MORBO_MAILER_FROM_NAME").unwrap();
        let from_email = env::var("MORBO_MAILER_FROM_EMAIL").unwrap();
        let to_name = env::var("MORBO_MAILER_TO_NAME").unwrap();
        let to_email = env::var("MORBO_MAILER_TO_EMAIL").unwrap();

        let smtp_hostname = env::var("MORBO_MAILER_SMTP_HOSTNAME").unwrap();
        let smtp_port = env::var("MORBO_MAILER_SMTP_PORT").unwrap().parse().unwrap();
        let smtp_username = env::var("MORBO_MAILER_SMTP_USERNAME").unwrap();
        let smtp_password = env::var("MORBO_MAILER_SMTP_PASSWORD").unwrap();

        Mailer {
            from_name,
            from_email,
            to_name,
            to_email,
            smtp_hostname,
            smtp_port,
            smtp_username: Some(smtp_username),
            smtp_password: Some(smtp_password),
        }
    }

    fn send_report(&self, report: &CspReportContent) -> Result<SmtpResult, Error> {
        let email = EmailBuilder::new()
            .from((
                &self.from_email,
                &self.from_name,
            ))
            .to((&self.to_email, &self.to_name))
            .subject("[CSP] New report")
            .body(format!(
                "New report {}",
                serde_json::to_string_pretty(report).unwrap()
            ))
            .build()
            .unwrap();

        let mut transport = self.get_transport().unwrap();
        Ok(transport.send(email.into()))
    }
}
