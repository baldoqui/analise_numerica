mod iterative_methods;
mod cli;

use env_logger;
use log::{info, error, LevelFilter};
use std::str::FromStr;
use clap::Parser;

use iterative_methods::jacobi::jacobi_iteration;
use cli::Args;

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(LevelFilter::from_str(&args.log_level).unwrap_or(LevelFilter::Info))
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();


    let a: Vec<Vec<f64>> = vec![
        vec![10.0, -1.0, 2.0, 0.0],
        vec![-1.0, 11.0, -1.0, 3.0],
        vec![2.0, -1.0, 10.0, -1.0],
        vec![0.0, 3.0, -1.0, 8.0],
    ];
    let b: Vec<f64> = vec![6.0, 25.0, -11.0, 15.0];
    
    let x0: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0];
    let tol: f64 = 1e-5;
    let max_iter: usize = 100;

    println!("Resolvendo o sistema Ax=b com a Iteração de Jacobi");
    println!("Matrix A: {:?}", a);
    println!("Vetor b: {:?}", b);
    println!("Tolerância: {}, Máx. Iterações: {}\n", tol, max_iter);

    match jacobi_iteration(&a, &b, &x0, tol, max_iter) {
        Ok(solution) => {
            println!("Solução convergiu:");
            for (i, val) in solution.iter().enumerate() {
                println!("  x[{}] = {:.6}", i, val);
            }
        }
        Err(e) => {
            eprintln!("\nErro: {}", e);
        }
    }
}