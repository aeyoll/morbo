use tide::Request;

#[macro_use]
extern crate clap;
use clap::App;

pub mod lib;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use crate::lib::csp_report::CspReport;
use crate::lib::mailer::Mailer;
use crate::lib::mailer_configuration::MailerConfiguration;

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

    let _res = csp_report.send_email(&mailer, &to_email, &to_name)?;

    Ok(format!(
        "CSP report: {}",
        serde_json::to_string_pretty(&csp_report).unwrap()
    )
    .into())
}
