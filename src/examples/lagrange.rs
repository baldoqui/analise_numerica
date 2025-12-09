use crate::iterative_methods::lagrange::lagrange_interpolator;

pub fn run() {
    println!("--- Running Lagrange Interpolation Example ---");

    let points = vec![
        (1.0, 0.0),
        (1.1, 0.271828),
        (1.2, 0.684756),
        (1.3, 1.276978),
        (1.4, 2.093548),
        (1.5, 3.187445),
        (1.6, 4.620818),
        (1.7, 6.466396),
        (1.8, 8.809120),
        (1.9, 11.747997),
        (2.0, 15.398236),
    ];

    for (x, y) in &points {
        println!("  ({:.1}, {:.1})", x, y);
    }
    println!();

    let p = lagrange_interpolator(&points);

    // --- Interpolation ---
    println!("Interpolating at new points:");
    let test_points = vec![1.04, 1.55, 1.97];
    for &x_new in &test_points {
        let y_new = p(x_new);
        println!("  p({:.3}) = {:.4}", x_new, y_new,);
    }

    println!("\n--- Lagrange Interpolation Example Finished ---\n");
}
