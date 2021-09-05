use tide::security::{CorsMiddleware, Origin};
use tide::{Request, StatusCode};

#[macro_use]
extern crate clap;
use clap::App;

pub mod csp;
pub mod channel;

#[cfg(feature = "mail")]
use channel::mailer::Mailer;

#[cfg(feature = "sentry")]
use channel::sentry::Sentry;

extern crate dotenv;
use dotenv::dotenv;

use crate::csp::csp_report::CspReport;
use crate::channel::channel::Channel;

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
        #[cfg(feature = "mail")]
        let _ = || {
            let mailer = Mailer::load_from_env();
            let _res = mailer.send_report(&csp_report).unwrap();
        };

        #[cfg(feature = "sentry")]
        let _ = || {
            let sentry = Sentry::load_from_env();
            let _res = sentry.send_report(&csp_report).unwrap();
        };

        Ok(format!(
            "CSP report: {}",
            serde_json::to_string_pretty(&csp_report).unwrap()
        )
        .into())
    } else {
        Err(tide::Error::from_str(StatusCode::Forbidden, ""))
    }
}
