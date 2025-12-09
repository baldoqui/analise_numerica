pub fn euler_with_error(
    f: &dyn Fn(f64, f64) -> f64,
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
        y += h * f(x, y);
        x += h;

        let y_analytical = analytical_sol(x);
        let error = (y_analytical - y).abs();
        results.push((x, y, y_analytical, error));
    }

    results
}
