#[derive(Debug, Default)]
pub struct MailerConfiguration {
    pub smtp_hostname: String,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
}
