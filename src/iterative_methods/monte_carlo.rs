use rand::Rng;

pub fn monte_carlo<F>(inside_fn: F, n_points: usize) -> f64
where
    F: Fn(f64, f64) -> bool,
{
    let mut rng = rand::rng();
    let mut inside_count = 0;

    for _ in 0..n_points {
        let x: f64 = rng.random();
        let y: f64 = rng.random();

        if inside_fn(x, y) {
            inside_count += 1;
        }
    }

    inside_count as f64 / n_points as f64
}
