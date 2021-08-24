#[derive(Debug, Default)]
pub struct MailConfiguration {
    pub to_name: String,
    pub to_email: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
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
