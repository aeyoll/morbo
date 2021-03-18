use tide::Request;

#[macro_use]
extern crate clap;
use clap::App;

mod csp_report;
mod csp_report_content;

use crate::csp_report::CspReport;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let port = matches.value_of("port").unwrap_or("8080");
    let binding = format!("127.0.0.1:{}", port);
    println!("Launching server on {}", binding);

    let mut app = tide::new();
    app.at("/_/csp-reports").post(csp_report);
    app.listen(&binding).await?;
    Ok(())
}

async fn csp_report(mut req: Request<()>) -> tide::Result {
    let CspReport { csp_report } = req.body_json().await?;
    Ok(format!(
        "CSP report: {}",
        serde_json::to_string_pretty(&csp_report).unwrap()
    )
    .into())
}
