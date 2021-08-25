use tide::security::{CorsMiddleware, Origin};
use tide::{Request, StatusCode};

#[macro_use]
extern crate clap;
use clap::App;

pub mod csp;
pub mod mailer;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use crate::csp::csp_report::CspReport;
use crate::mailer::mailer::Mailer;
use crate::mailer::mailer_configuration::MailerConfiguration;

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

    let cors = CorsMiddleware::new()
        .allow_methods(
            "GET, POST, PUT, OPTIONS"
                .parse::<tide::http::headers::HeaderValue>()
                .unwrap(),
        )
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    app.at("/_/csp-reports").post(csp_report_action);
    app.listen(&binding).await?;
    Ok(())
}

async fn csp_report_action(mut req: Request<()>) -> tide::Result {
    let CspReport { csp_report } = req.body_json().await?;

    if !csp_report.is_in_block_list() {
        let to_name = env::var("TO_NAME").unwrap();
        let to_email = env::var("TO_EMAIL").unwrap();

        let smtp_hostname = env::var("SMTP_HOSTNAME").unwrap();
        let smtp_port = env::var("SMTP_PORT").unwrap().parse().unwrap();
        let smtp_username = env::var("SMTP_USERNAME").unwrap();
        let smtp_password = env::var("SMTP_PASSWORD").unwrap();

        let mailer_configuration = MailerConfiguration {
            smtp_hostname,
            smtp_port,
            smtp_username: Some(smtp_username),
            smtp_password: Some(smtp_password),
        };

        let mailer = Mailer {
            configuration: mailer_configuration,
        };

        let _res = mailer.send_report(&csp_report, &to_email, &to_name)?;

        Ok(format!(
            "CSP report: {}",
            serde_json::to_string_pretty(&csp_report).unwrap()
        )
        .into())
    } else {
        Err(tide::Error::from_str(StatusCode::Forbidden, ""))
    }
}
