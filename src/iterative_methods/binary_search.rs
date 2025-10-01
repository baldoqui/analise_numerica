pub fn binary_search<F>(
    f: F,
    mut a: f64,
    mut b: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    F: Fn(f64) -> f64,
{
    if f(a) * f(b) >= 0.0 {
        return Err("A função deve ter sinais opostos nos pontos a e b.".to_string());
    }

    let mut iter = 0;
    while (b - a) / 2.0 > tol && iter < max_iter {
        let c = (a + b) / 2.0;
        if f(c) == 0.0 {
            return Ok(c);
        } else if f(a) * f(c) < 0.0 {
            b = c;
        } else {
            a = c;
        }
        iter += 1;
    }

    Ok((a + b) / 2.0)
}
