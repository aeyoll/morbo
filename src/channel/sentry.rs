use crate::csp::csp_report_content::CspReportContent;
use sentry_core;
use anyhow::Error;
use std::env;
use crate::channel::channel::Channel;
use sentry_core::types::Uuid;

/// Sentry channel
pub struct Sentry {
    pub dsn: String,
}

impl Channel<Uuid> for Sentry {
    fn load_from_env() -> Self {
        let dsn = env::var("MORBO_SENTRY_DSN").unwrap();

        Sentry { dsn }
    }

    fn send_report(&self, report: &CspReportContent) -> Result<Uuid, Error> {
        let dsn = self.dsn.as_str();
        let _guard = sentry_core::init(dsn);
        let report = serde_json::to_string_pretty(report).unwrap();

        Ok(sentry_core::capture_message(&*report, sentry_core::Level::Info))
    }
}