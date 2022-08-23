use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Running port
    #[clap(short, long, value_parser, default_value_t = 8080)]
    pub port: u16,

    // Be verbose during process
    #[clap(short, long)]
    pub verbose: bool,

    // Print additionals debug informations
    #[clap(short, long)]
    pub debug: bool,
}
