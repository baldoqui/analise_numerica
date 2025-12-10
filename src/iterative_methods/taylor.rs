pub fn taylor_order_2_with_error(
    f: &dyn Fn(f64, f64) -> f64,
    f_prime: &dyn Fn(f64, f64) -> f64,
    analytical_sol: &dyn Fn(f64) -> f64,
    x0: f64,
    y0: f64,
    h: f64,
    n: usize,
) -> Vec<(f64, f64, f64, f64)> {
    let mut results = Vec::with_capacity(n + 1);
    let mut x = x0;
    let mut y = y0;

    // Store initial values
    let y_analytical = analytical_sol(x);
    let error = (y_analytical - y).abs();
    results.push((x, y, y_analytical, error));

    for _ in 0..n {
        y += h * f(x, y) + (h.powi(2) / 2.0) * f_prime(x, y);
        x += h;

        let y_analytical = analytical_sol(x);
        let error = (y_analytical - y).abs();
        results.push((x, y, y_analytical, error));
    }

    results
}

pub fn taylor_order_4_with_error(
    f: &dyn Fn(f64, f64) -> f64,
    f_prime: &dyn Fn(f64, f64) -> f64,
    f_second: &dyn Fn(f64, f64) -> f64,
    f_third: &dyn Fn(f64, f64) -> f64,
    analytical_sol: &dyn Fn(f64) -> f64,
    x0: f64,
    y0: f64,
    h: f64,
    n: usize,
) -> Vec<(f64, f64, f64, f64)> {
    let mut results = Vec::with_capacity(n + 1);
    let mut x = x0;
    let mut y = y0;

    let y_analytical = analytical_sol(x);
    let error = (y_analytical - y).abs();
    results.push((x, y, y_analytical, error));

    for _ in 0..n {
        let y_prime = f(x, y);
        let y_prime2 = f_prime(x, y);
        let y_prime3 = f_second(x, y);
        let y_prime4 = f_third(x, y);

        y += h * y_prime
            + (h.powi(2) / 2.0) * y_prime2
            + (h.powi(3) / 6.0) * y_prime3
            + (h.powi(4) / 24.0) * y_prime4;

        x += h;

        let y_analytical = analytical_sol(x);
        let error = (y_analytical - y).abs();
        results.push((x, y, y_analytical, error));
    }

    results
}
