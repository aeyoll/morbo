use tide::security::{CorsMiddleware, Origin};
use tide::{Request, StatusCode};
use tide::log;

#[macro_use]
extern crate clap;
use clap::App;

pub mod channel;
pub mod csp;

#[cfg(feature = "mail")]
use channel::mailer::Mailer;

#[cfg(feature = "sentry")]
use channel::sentry::Sentry;

extern crate dotenv;
use dotenv::dotenv;

use crate::csp::csp_report::CspReport;

#[cfg(any(feature = "mail", feature = "sentry"))]
use crate::channel::channel::Channel;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Load env variables
    dotenv().ok();

    // Cli args
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("debug") {
        log::with_level(log::LevelFilter::Debug);
    } else if matches.is_present("verbose") {
        log::with_level(log::LevelFilter::Info);
    } else {
        log::with_level(log::LevelFilter::Warn);
    }

    let port = matches.value_of("port").unwrap_or("8080");
    let binding = format!("127.0.0.1:{}", port);
    log::info!("Launching server on {}", binding);
    let vars: Vec<(String, String)> = dotenv::vars().filter(|x| x.0.starts_with("MORBO_")).collect();
    log::debug!("environment: {:?}", vars);

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
    log::info!("Received a new report");
    let CspReport { csp_report } = req.body_json().await?;

    if !csp_report.is_in_block_list() {
        #[cfg(feature = "mail")] {
            log::debug!("Sending report by email");
            let mailer = Mailer::load_from_env();
            let _res = mailer.send_report(&csp_report).unwrap();
        };

        #[cfg(feature = "sentry")] {
            log::debug!("Sending report to sentry");
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
