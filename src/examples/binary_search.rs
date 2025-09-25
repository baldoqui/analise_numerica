
use crate::iterative_methods::binary_search::binary_search;

pub fn run() {
    println!("Executando exemplo da busca binária...");

    let f = |x: f64| x.powi(3) - x - 2.0;
    let a = 1.0;
    let b = 2.0;
    let tol = 1e-6;
    let max_iter = 100;

    match binary_search(f, a, b, tol, max_iter) {
        Ok(root) => {
            println!("A raiz encontrada é: {:.6}", root);
            println!("f({}) = {:.6}", root, f(root));
        }
        Err(e) => println!("Erro: {}", e),
    }
}
