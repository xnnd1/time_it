use clap::Parser;
use log::{info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub exec_path: String,
    pub exec_args: Option<Vec<String>>
}

pub fn parse_args() -> Args {
    let args = Args::parse();

    info!("\nexec_path argument: {:?}\nexec_args argument: {:?}", args.exec_path, args.exec_args);

    args
}