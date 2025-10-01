pub fn newton<F, FPrime>(
    f: F,
    f_prime: FPrime,
    mut x0: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    F: Fn(f64) -> f64,
    FPrime: Fn(f64) -> f64,
{
    for _ in 0..max_iter {
        let y = f(x0);
        let y_prime = f_prime(x0);

        if y_prime.abs() < 1e-12 {
            return Err("Derivada próxima de zero.".to_string());
        }

        let x1 = x0 - y / y_prime;

        if (x1 - x0).abs() < tol {
            return Ok(x1);
        }

        x0 = x1;
    }

    Err(format!("O método não convergiu em {} iterações.", max_iter))
}
