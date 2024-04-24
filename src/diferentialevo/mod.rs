use crate::particle::Particle;
use rand::Rng;

fn add(v1: Vec<f64>, v2: Vec<f64>) -> Vec<f64> {
    let mut result = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        result.push(v1[i] + v2[i]);
    }
    result
}

fn sub(v1: Vec<f64>, v2: Vec<f64>) -> Vec<f64> {
    let mut result = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        result.push(v1[i] - v2[i]);
    }
    result
}

fn mul4scalar(v: Vec<f64>, s: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        result.push(v[i] * s);
    }
    result
}

fn mutation(v: Vec<Vec<f64>>, f: f64) -> Vec<f64> {
    let xr1 = v[0].clone();
    let xr2 = v[1].clone();
    let xr3 = v[2].clone();

    let vmuted = add(xr1, mul4scalar(sub(xr2, xr3), f));
    vmuted
}

fn crossver(vmuted: Vec<f64>, voriginal: Vec<f64>) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let j = rng.gen_range(0..voriginal.len());
    let mut v = vec![];
    for i in 0..voriginal.len() {
        if i == j {
            v.push(vmuted[i]);
        } else {
            v.push(voriginal[i]);
        }
    }
    v
}

pub fn initialize_population(num_p: usize, num_d: usize, range: (f64, f64)) -> Vec<Particle> {
    let mut population = Vec::with_capacity(num_p);
    let mut rng = rand::thread_rng();

    for _ in 0..num_p {
        let mut position = Vec::with_capacity(num_d);
        for _ in 0..num_d {
            let x = rng.gen_range(range.0..range.1);
            position.push(x);
        }
        let particle = Particle::new(position);
        population.push(particle);
    }
    population
}

fn obj(position: Vec<f64>) -> Vec<f64> {
    position
}

pub struct Diferentialevo {
    solutions: Vec<Particle>,
    problem: String,
    max_iter: usize,
    constraints: fn(Vec<f64>) -> Vec<f64>,
    objective: fn(Vec<f64>) -> f64,
    best_solution: Particle,
    worst_solution: Particle,
}

impl Diferentialevo {
    pub fn default(
        solutions: Vec<Particle>,
        problem: String,
        max_iter: usize,
        objective: fn(Vec<f64>) -> f64,
    ) -> Diferentialevo {
        Diferentialevo {
            solutions,
            problem,
            max_iter,
            constraints: obj,
            objective,
            best_solution: Particle::new(vec![]),
            worst_solution: Particle::new(vec![]),
        }
    }

    #[allow(dead_code)]
    pub fn with_constraints(
        solutions: Vec<Particle>,
        problem: String,
        max_iter: usize,
        constraints: fn(Vec<f64>) -> Vec<f64>,
        objective: fn(Vec<f64>) -> f64,
    ) -> Diferentialevo {
        Diferentialevo {
            solutions,
            problem,
            max_iter,
            constraints,
            objective,
            best_solution: Particle::new(vec![]),
            worst_solution: Particle::new(vec![]),
        }
    }

    pub fn update_avalue(&mut self) {
        for s in &mut self.solutions {
            s.position = (self.constraints)(s.position.clone());
            s.avalue = (self.objective)(s.position.clone());
        }

        if self.problem == "min" {
            self.solutions
                .sort_by(|a, b| a.avalue.partial_cmp(&b.avalue).unwrap());
            let half_size = self.solutions.len() / 2;
            self.solutions.truncate(half_size);
        } else {
            self.solutions
                .sort_by(|a, b| b.avalue.partial_cmp(&a.avalue).unwrap());
            let half_size = self.solutions.len() / 2;
            self.solutions.truncate(half_size);
        }
    }

    pub fn update_best_worst(&mut self) {
        if self.problem == "min" {
            self.best_solution = self
                .solutions
                .iter()
                .min_by(|a, b| a.avalue.partial_cmp(&b.avalue).unwrap())
                .unwrap()
                .clone();
            self.worst_solution = self
                .solutions
                .iter()
                .max_by(|a, b| a.avalue.partial_cmp(&b.avalue).unwrap())
                .unwrap()
                .clone();
        } else {
            self.best_solution = self
                .solutions
                .iter()
                .max_by(|a, b| a.avalue.partial_cmp(&b.avalue).unwrap())
                .unwrap()
                .clone();
            self.worst_solution = self
                .solutions
                .iter()
                .min_by(|a, b| a.avalue.partial_cmp(&b.avalue).unwrap())
                .unwrap()
                .clone();
        }
    }

    pub fn show_worst(&self) {
        self.worst_solution.print();
    }

    fn vec_4_mut(&self) -> Vec<Vec<f64>> {
        let mut rng = rand::thread_rng();
        let mut r1 = rng.gen_range(0..self.solutions.len());
        let mut r2 = rng.gen_range(0..self.solutions.len());
        let mut r3 = rng.gen_range(0..self.solutions.len());

        while r1 == r2 || r1 == r3 || r2 == r3 {
            r1 = rng.gen_range(0..self.solutions.len());
            r2 = rng.gen_range(0..self.solutions.len());
            r3 = rng.gen_range(0..self.solutions.len());
        }

        let mut v = vec![];
        v.push(self.solutions[r1].position.clone());
        v.push(self.solutions[r2].position.clone());
        v.push(self.solutions[r3].position.clone());

        return v;
    }

    pub fn run(&mut self)->Result<Particle, &'static str>{
        for _ in 0..self.max_iter {
            println!("Numero de soluciones: {}", self.solutions.len());
            for s in &self.solutions.clone() {
                let v: Vec<Vec<f64>> = self.vec_4_mut();
                let vmuted = mutation(v, 0.5);
                let mut vtrial = crossver(vmuted, s.position.clone());
                vtrial = (self.constraints)(vtrial);
                let new_solution = Particle::new(vtrial);
                self.solutions.push(new_solution);
            }
            println!("El numero de soluciones antes de seleccionar es: {}", self.solutions.len());
            self.update_avalue();
            self.update_best_worst();
            println!("El numero de soluciones despues de seleccionar es: {}", self.solutions.len());
            
        }
        Ok(self.best_solution.clone())
    }
}
