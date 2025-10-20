use crate::iterative_methods::monte_carlo::monte_carlo;

pub fn run() {
    let n = 10_000;
    let ratio = monte_carlo(|x, y| x * x + y * y <= 1.0, n);

    let pi = ratio * 4.0;

    println!("Estimativa de pi com {} pontos: {:.8}", n, pi)
}
