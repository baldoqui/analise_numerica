use crate::iterative_methods::newton::newton;

pub fn run() {
    let f = |x: f64| (x - 1.0).ln() + (x - 1.0).cos();
    let f_prime = |x: f64| (1.0 / (x - 1.0)) - (x - 1.0).sin();
    let x0 = 1.6;
    let tol = 1e-5;
    let max_iter = 100;

    match newton(f, f_prime, x0, tol, max_iter) {
        Ok(root) => {
            println!("A raiz encontrada é: {:.6}", root);
            println!("f({}) = {:.6}", root, f(root));
        }
        Err(e) => println!("Erro: {}", e),
    }
}
