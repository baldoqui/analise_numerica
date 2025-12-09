use crate::iterative_methods::euler::euler_with_error;

use std::f64::consts::E;

pub fn run() {
    println!("--- Running Euler's Method Example ---");

    let f = |t: f64, y: f64| (2.0 / t) * y + t.powi(2) * t.exp();

    let analytical_sol = |x: f64| x.powi(2) * (x.exp() - E);

    let x0 = 1.0;
    let y0 = 0.0;
    let h = 0.1;
    let n = 10;

    println!(
        "Solving y' = t*e^3t - 2y with y({}) = {}, h = {}",
        x0, y0, h
    );
    println!("Comparing with analytical solution\n");

    let results = euler_with_error(&f, &analytical_sol, x0, y0, h, n);

    // Print header
    println!(
        "{:<5} {:<10} {:<20} {:<20} {:<20}",
        "Step", "x", "y (Approx)", "y (Analytical)", "Error"
    );
    println!("{:-<85}", "");

    for (i, (x, y_approx, y_analytical, error)) in results.iter().enumerate() {
        println!(
            "{:<5} {:<10.2} {:<20.6} {:<20.6} {:<20.6}",
            i, x, y_approx, y_analytical, error
        );
    }
    println!("\n--- Euler's Method Example Finished ---\n");
}
