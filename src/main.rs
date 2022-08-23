use std::net::SocketAddr;

mod args;
mod channel;
mod csp;

use axum::http::{HeaderValue, Method};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

#[cfg(feature = "mail")]
use channel::mailer::Mailer;

#[cfg(feature = "sentry")]
use channel::sentry::Sentry;

extern crate dotenv;
use dotenv::dotenv;

use crate::csp::csp_report::CspReport;

#[cfg(any(feature = "mail", feature = "sentry"))]
use crate::channel::channel::Channel;

use tower_http::cors::CorsLayer;

use crate::args::Args;
use clap::Parser;

#[tokio::main]
async fn main() {
    // Load env variables
    dotenv().ok();

    // Cli args
    let args = Args::parse();

    if args.is_present("debug") {
        log::with_level(log::LevelFilter::Debug);
    } else if args.is_present("verbose") {
        log::with_level(log::LevelFilter::Info);
    } else {
        log::with_level(log::LevelFilter::Warn);
    }

    // initialize tracing
    tracing_subscriber::fmt::init();

    // App initialisation
    let app = Router::new()
        .route("/_/csp-reports", post(csp_report_action))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::OPTIONS])
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_credentials(false),
        );

    // Run it
    let port: u16 = args.port;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn csp_report_action(Json(payload): Json<CspReport>) -> impl IntoResponse {
    let csp_report = payload.csp_report;

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
    }

    (StatusCode::OK, Json(""))
}
