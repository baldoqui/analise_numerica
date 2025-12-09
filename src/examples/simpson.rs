use crate::iterative_methods::simpson;
use log::{info, error};
use std::f64::consts::PI;

pub fn run(function_name: String) {
    match function_name.as_str() {
        "f1" => {
            // integral de 1 a 2 de x*ln(x)dx com n=4
            let f = |x: f64| x * x.ln();
            let a = 1.0;
            let b = 2.0;
            let n = 4;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of x*ln(x) from 1 to 2 (n=4): {}", integral);
        },
        "f2" => {
            // integral de -2 a 2 de x^3*e^x dx com n=4
            let f = |x: f64| x.powi(3) * x.exp();
            let a = -2.0;
            let b = 2.0;
            let n = 4;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of x^3*e^x from -2 to 2 (n=4): {}", integral);
        },
        "f3" => {
            // integral de 0 a 2 de 2/(x^2+4)dx com n=6
            let f = |x: f64| 2.0 / (x.powi(2) + 4.0);
            let a = 0.0;
            let b = 2.0;
            let n = 6;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of 2/(x^2+4) from 0 to 2 (n=6): {}", integral);
        },
        "f4" => {
            // integral de 0 a pi de x^2*cos(x)dx com n=6
            let f = |x: f64| x.powi(2) * x.cos();
            let a = 0.0;
            let b = PI;
            let n = 6;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of x^2*cos(x) from 0 to pi (n=6): {}", integral);
        },
        "f5" => {
            // integral de 0 a 2 de e^(2x)*sen(3x) dx, com n=8
            let f = |x: f64| (2.0 * x).exp() * (3.0 * x).sin();
            let a = 0.0;
            let b = 2.0;
            let n = 8;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of e^(2x)*sin(3x) from 0 to 2 (n=8): {}", integral);
        },
        "f6" => {
            // integral de 1 a 3 de x/(x^2+4) dx com n=8
            let f = |x: f64| x / (x.powi(2) + 4.0);
            let a = 1.0;
            let b = 3.0;
            let n = 8;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of x/(x^2+4) from 1 to 3 (n=8): {}", integral);
        },
        "f7" => {
            // integral de 3 a 5 de 1/sqrt(x^2-4) dx com n=8
            let f = |x: f64| 1.0 / (x.powi(2) - 4.0).sqrt();
            let a = 3.0;
            let b = 5.0;
            let n = 8;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of 1/sqrt(x^2-4) from 3 to 5 (n=8): {}", integral);
        },
        "f8" => {
            // integral de 0 a 3pi/8 de tg(x) dx com n=8
            let f = |x: f64| x.tan();
            let a = 0.0;
            let b = 3.0 * PI / 8.0;
            let n = 8;
            let integral = simpson::integrate_simpson(f, a, b, n);
            info!("Integral of tan(x) from 0 to 3pi/8 (n=8): {}", integral);
        },
        _ => {
            error!("Unknown function: {}. Please use f1, f2, ..., f8.", function_name);
        }
    }
}