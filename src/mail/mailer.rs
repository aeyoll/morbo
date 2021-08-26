use crate::csp::csp_report_content::CspReportContent;
use crate::mail::mailer_configuration::MailerConfiguration;
use anyhow::Error;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::SmtpResult;
use lettre::{ClientSecurity, SmtpClient, SmtpTransport};
use lettre_email::EmailBuilder;

use lettre::Transport;

pub struct Mailer {
    pub configuration: MailerConfiguration,
}

impl Mailer {
    pub fn get_transport(&self) -> Result<SmtpTransport, Error> {
        let addr = (
            self.configuration.smtp_hostname.as_str(),
            self.configuration.smtp_port,
        );
        let security = ClientSecurity::None;

        let mut smtp_client = SmtpClient::new(addr, security)?;

        if self.configuration.smtp_username.is_some() && self.configuration.smtp_password.is_some()
        {
            let credentials = Credentials::new(
                self.configuration
                    .smtp_username
                    .as_ref()
                    .unwrap()
                    .to_owned(),
                self.configuration
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

    pub fn send_report(
        &self,
        report: &CspReportContent,
    ) -> Result<SmtpResult, Error> {
        let email = EmailBuilder::new()
            .from((&self.configuration.from_email, &self.configuration.from_name))
            .to((&self.configuration.to_email, &self.configuration.to_name))
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
