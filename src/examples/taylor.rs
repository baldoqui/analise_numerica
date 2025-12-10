use std::{f64::consts::E, iter};

use crate::iterative_methods::{
    lagrange::lagrange_interpolator, taylor::taylor_order_2_with_error,
};

pub fn run() {
    println!("--- Running Taylor Method (Order 2) Example ---");

    let f = |t: f64, y: f64| (2.0 / t) * y + t.powi(2) * t.exp();

    let f_prime = |t: f64, y: f64| (2.0 / t.powi(2)) * y + t.exp() * (4.0 * t + t.powi(2));

    let analytical_sol = |t: f64| t.powi(2) * (t.exp() - E);

    let x0 = 1.0;
    let y0 = 0.0;
    let h = 0.1;
    let n = 10;

    let results = taylor_order_2_with_error(&f, &f_prime, &analytical_sol, x0, y0, h, n);

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
    println!("\n--- Taylor Method Example Finished ---\n");

    let points: Vec<(f64, f64)> = results.into_iter().map(|r| (r.0, r.1)).collect();
    let p = lagrange_interpolator(&points);

    let y1 = p(1.04);
    let y2 = p(1.55);
    let y3 = p(1.97);

    println!("y1 = {:<20.6} y2 = {:<20.6} y3 = {:<20.6}", y1, y2, y3);
}
