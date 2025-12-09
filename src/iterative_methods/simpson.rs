pub fn integrate_simpson<F>(f: F, a: f64, b: f64, n: u64) -> f64
where
    F: Fn(f64) -> f64,
{
    if n % 2 != 0 {
        panic!("n must be an even number for Simpson's rule");
    }

    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 0 {
            sum += 2.0 * f(x);
        } else {
            sum += 4.0 * f(x);
        }
    }

    (h / 3.0) * sum
}
