use crate::iterative_methods::taylor::taylor_order_4_with_error;
use std::f64::consts::E;

pub fn run() {
    println!("--- Running Taylor Method (Order 4) Example ---");

    let f = |t: f64, y: f64| (2.0 / t) * y + t.powi(2) * t.exp();

    let f_prime = |t: f64, y: f64| (2.0 / t.powi(2)) * y + t.exp() * (4.0 * t + t.powi(2));

    let f_second = |t: f64, _: f64| t.exp() * (6.0 + 6.0 * t + t.powi(2));

    let f_third = |t: f64, _: f64| t.exp() * (12.0 + 8.0 * t + t.powi(2));

    let analytical_sol = |t: f64| t.powi(2) * (t.exp() - E);

    let x0 = 1.0;
    let y0 = 0.0;
    let h = 0.1;
    let n = 10;

    let results = taylor_order_4_with_error(
        &f,
        &f_prime,
        &f_second,
        &f_third,
        &analytical_sol,
        x0,
        y0,
        h,
        n,
    );

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
    println!("\n--- Taylor Method (Order 4) Example Finished ---\n");
}
