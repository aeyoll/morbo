use std::env;

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

impl MailerConfiguration {
    pub fn load_from_env() -> Self {
        let from_name = env::var("MORBO_FROM_NAME").unwrap();
        let from_email = env::var("MORBO_FROM_EMAIL").unwrap();
        let to_name = env::var("MORBO_TO_NAME").unwrap();
        let to_email = env::var("MORBO_TO_EMAIL").unwrap();

        let smtp_hostname = env::var("MORBO_SMTP_HOSTNAME").unwrap();
        let smtp_port = env::var("MORBO_SMTP_PORT").unwrap().parse().unwrap();
        let smtp_username = env::var("MORBO_SMTP_USERNAME").unwrap();
        let smtp_password = env::var("MORBO_SMTP_PASSWORD").unwrap();

        MailerConfiguration {
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
}
