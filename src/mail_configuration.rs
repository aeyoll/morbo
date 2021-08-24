#[derive(Debug, Default)]
pub struct MailConfiguration {
    pub to_name: String,
    pub to_email: String,
    pub smtp_hostname: String,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
}

impl MailConfiguration {}
