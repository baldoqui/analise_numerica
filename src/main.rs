mod cli;
mod examples;
mod iterative_methods;

use clap::Parser;
use env_logger;
use log::LevelFilter;
use std::str::FromStr;

use cli::{Args, Commands};
use examples::{
    binary_search, euler, jacobi, lagrange, monte_carlo_pi, newton, simpson, taylor,
    taylor_order_4, trapezoid,
};

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(LevelFilter::from_str(&args.log_level).unwrap_or(LevelFilter::Info))
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    match args.command {
        Commands::Jacobi => jacobi::run(),
        Commands::BinarySearch => binary_search::run(),
        Commands::Newton => newton::run(),
        Commands::MonteCarlo => monte_carlo_pi::run(),
        Commands::Trapezoid => trapezoid::run(),
        Commands::Simpson { function } => simpson::run(function),
        Commands::Euler => euler::run(),
        Commands::Lagrange => lagrange::run(),
        Commands::Taylor => taylor::run(),
        Commands::TaylorOrder4 => taylor_order_4::run(),
    }
}
