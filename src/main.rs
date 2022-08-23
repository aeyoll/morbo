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
use crate::channel::Channel;

use tower_http::cors::CorsLayer;

use crate::args::Args;
use clap::Parser;
use tracing::Level;
use tracing::log::debug;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Load env variables
    dotenv().ok();

    // Cli args
    let args = Args::parse();

    // initialize tracing
    let mut max_level = Level::WARN;

    if args.debug {
        max_level = Level::DEBUG;
    } else if args.verbose {
        max_level = Level::INFO;
    }

    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn csp_report_action(Json(payload): Json<CspReport>) -> impl IntoResponse {
    let csp_report = payload.csp_report;

    if !csp_report.is_in_block_list() {
        #[cfg(feature = "mail")]
        {
            debug!("Sending report by email");
            let mailer = Mailer::load_from_env();
            let _res = mailer.send_report(&csp_report).unwrap();
        };

        #[cfg(feature = "sentry")]
        {
            debug!("Sending report to sentry");
            let sentry = Sentry::load_from_env();
            let _res = sentry.send_report(&csp_report).unwrap();
        };
    }

    (StatusCode::OK, Json(""))
}
