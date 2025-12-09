pub fn lagrange_interpolator(points: &[(f64, f64)]) -> Box<dyn Fn(f64) -> f64> {
    let owned_points = points.to_vec();

    Box::new(move |x: f64| -> f64 {
        let mut total_sum = 0.0;

        for (j, (xj, yj)) in owned_points.iter().enumerate() {
            let mut basis_product = 1.0;
            for (m, (xm, _)) in owned_points.iter().enumerate() {
                if j != m {
                    if (xj - xm).abs() < 1e-10 {
                        panic!(
                            "Two points have the same x-coordinate, which is not allowed for Lagrange interpolation."
                        );
                    }
                    basis_product *= (x - xm) / (xj - xm);
                }
            }
            total_sum += yj * basis_product;
        }
        total_sum
    })
}
