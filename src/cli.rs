use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "info")]
    pub log_level: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "kebab-case")]
pub enum Commands {
    Jacobi,
    BinarySearch,
    Newton,
    MonteCarlo,
    Trapezoid,
    Simpson {
        #[arg(long)]
        function: String,
    },
    Euler,
    Lagrange,
    Taylor,
    TaylorOrder4,
}
