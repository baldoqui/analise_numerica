use crate::iterative_methods::jacobi::jacobi_iteration;
use log::{error, info};

pub fn run() {
    let a = vec![
        vec![10.0, -1.0, 2.0, 0.0],
        vec![-1.0, 11.0, -1.0, 3.0],
        vec![2.0, -1.0, 10.0, -1.0],
        vec![0.0, 3.0, -1.0, 8.0],
    ];
    let b = vec![6.0, 25.0, -11.0, 11.0];
    let x0 = vec![0.0, 0.0, 0.0, 0.0];
    let tol = 1e-5;
    let max_iter = 100;

    info!("Resolvendo o sistema Ax=b com a método de Jacobi");
    info!("Matrix A: {:?}", a);
    info!("Vetor b: {:?}", b);
    info!("Tolerância: {}, Máx. Iterações: {}\n", tol, max_iter);

    match jacobi_iteration(&a, &b, &x0, tol, max_iter) {
        Ok(x) => {
            info!("Solução aproximada: {:?}", x);
        }
        Err(e) => {
            error!("Erro: {}", e);
        }
    }
}
