use crate::iterative_methods::trapezoid;
use log::info;


fn f(x: f64) -> f64 {
    x.tan()
}

pub fn run() {
    let pi = 3.1415926536;
    let a = 0.0;
    let b = 3.0*pi/8.0;
    let n = 8;

    let integral = trapezoid::integrate_trapezoid(f, a, b, n);

    info!("The integral of f(x) from {} to {} is: {}", a, b, integral);
}
