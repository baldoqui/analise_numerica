use log::{trace};

pub fn jacobi_iteration(
    a: &Vec<Vec<f64>>,
    b: &Vec<f64>,
    x0: &Vec<f64>,
    tol: f64,
    max_iter: usize,
) -> Result<Vec<f64>, String> {
    let n = a.len();
    let mut x = x0.clone();
    let mut x_new = vec![0.0; n];

    for k in 0..max_iter {
        trace!("Iteração {}: x = {:?}", k + 1, x);
        
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                if i != j {
                    sum += a[i][j] * x[j];
                }
            }

            if a[i][i].abs() < 1e-10 {
                return Err(format!("Elemento diagonal a[{}][{}] é zero.", i, i));
            }

            x_new[i] = (b[i] - sum) / a[i][i];
        }

        let mut norm: f64 = 0.0;
        for i in 0..n {
            norm = norm.max((x_new[i] - x[i]).abs());
        }

        if norm < tol {
            trace!("Convergência alcançada na iteração {}. x = {:?} \n", k + 1, x_new);
            return Ok(x_new);
        }

        x = x_new.clone();
    }

    Err(format!(
        "O método não convergiu em {} iterações.",
        max_iter
    ))
}
