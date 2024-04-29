mod differentialevo;
mod diffevomaze;
mod particle;
use colored::*;
use differentialevo::Differentialevo;
use diffevomaze::Diffevomaze;


// fn dixon_price(x: Vec<f64>) -> f64 {
//     let mut sum = 0.0;
//     for i in 1..x.len() {
//         let xi = x[i];
//         let xi_minus_1 = x[i - 1];
//         let term = i as f64 * (2.0 * xi.powi(2) - xi_minus_1).powi(2);
//         sum += term;
//     }
//     sum + (x[0] - 1.0).powi(2)
// }

// fn rosenbrock(x: Vec<f64>) -> f64 {
//     let total = (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0].powi(2)).powi(2);
//     total
// }

fn sphere(x: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.len() {
        sum += x[i].powi(2);
    }
    sum
}

fn main() {
    // let s = differentialevo::initialize_population(100, 2, (1.0, 5.0));
    // let mut de = Differentialevo::default(s, "min".to_string(), 20, sphere);
    // let best = de.run().unwrap();
    // println!("");
    // println!("{}", format!("Optimization finished").on_purple());
    // println!("{}", format!("Best solution:").on_green());
    // best.0.print();

    // println!("{}", format!("Worst solution:").on_red());
    // de.show_worst();

    let maze: Vec<Vec<usize>> = vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![2, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 4],
        vec![1, 1, 1, 1, 1, 1]
    ];

    let s = diffevomaze::initialize_vecs(5, 6, maze.clone());
    for i in 0..s.len() {
        s[i].print();
    }



}


