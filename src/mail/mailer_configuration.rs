#[derive(Debug, Default)]
pub struct MailerConfiguration {
    pub from_name: String,
    pub from_email: String,

    pub to_name: String,
    pub to_email: String,

    pub smtp_hostname: String,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
}
