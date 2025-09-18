mod cli;
mod examples;
mod iterative_methods;

use clap::Parser;
use env_logger;
use log::LevelFilter;
use std::str::FromStr;

use cli::Args;
use examples::jacobi;

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(LevelFilter::from_str(&args.log_level).unwrap_or(LevelFilter::Info))
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    jacobi::jacobi_example();
}
