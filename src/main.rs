mod diferentialevo;
mod particle;
use colored::*;
use diferentialevo::Diferentialevo;

pub fn dixon_price(x: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 1..x.len() {
        let xi = x[i];
        let xi_minus_1 = x[i - 1];
        let term = i as f64 * (2.0 * xi.powi(2) - xi_minus_1).powi(2);
        sum += term;
    }
    sum + (x[0] - 1.0).powi(2)
}

// fn rosenbrock(x: Vec<f64>) -> f64 {
//     let total = (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0].powi(2)).powi(2);
//     total
// }

fn main() {
    let s = diferentialevo::initialize_population(20, 2, (1.0, 5.0));
    let mut de = Diferentialevo::default(s, "min".to_string(), 100, dixon_price);
    let best = de.run().unwrap();
    println!("");
    println!("{}", format!("Optimization finished").on_purple());
    println!("{}", format!("Best solution:").on_green());
    best.print();
    println!("{}", format!("Worst solution:").on_red());
    de.show_worst();
}
