use tide::Request;

#[macro_use]
extern crate clap;
use clap::App;

mod csp_report;
mod csp_report_content;
mod mail_configuration;

use crate::csp_report::CspReport;
use crate::mail_configuration::MailConfiguration;

extern crate lettre;
extern crate lettre_email;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

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

    let mail_configuration = MailConfiguration::new(
        env::var("TO_NAME").unwrap(),
        env::var("TO_EMAIL").unwrap(),
        env::var("SMTP_SERVER").unwrap(),
        env::var("SMTP_USERNAME").unwrap(),
        env::var("SMTP_PASSWORD").unwrap(),
    );

    csp_report.send_email(&mail_configuration).unwrap();

    Ok(format!(
        "CSP report: {}",
        serde_json::to_string_pretty(&csp_report).unwrap()
    )
    .into())
}
