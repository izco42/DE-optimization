use crate::particle::Solution;
use colored::*;
use rand::Rng;
/*
El numero de pasos maximos por solucion es la cantidad de celdas en la matriz

la funcion evaluara como mejor solucion aquella que sea valida y su coste sea el menor

las dimensiones position de vec deben ser una tupla que sea una posicion en la matriz
toda la position es una secuencia de pasos para resolver el laberinto , las componentes de la tupla
deben ser inicializadas (x,y) en un rango de 1 a el tamanio de la matriz

se debe agregar un contador de soluciones validas que se detenga al encontrar la cantidad deseada
num_p , solo se agrega una solucion valida al vector de soluciones despues de verificar con
is_solution() (aun pendiente)

las operaciones de mutacion y crossover deben verificar los vectores recibidos , para operar
el mayor con el menor , evitando acceder a posicion no existente en el vector mas pequeño
(tal vez validar esto desde vec_4_mut() insertando primero en el vector que regresa aquel mas grande

despues de la mutacion y crossover se debe verificar si la nueva solucion es valida con is_solution()
en caso de que el numero de soluciones no sean suficientes , se vuelven a generar nuevas soluciones
para mantener el numero de la poblacion estable
*/
pub struct Diffevomaze {
    solutions: Vec<Solution>,
    problem: String,
    max_iter: usize,
    objective: fn(Vec<(usize, usize)>) -> f64,
    maze: Vec<Vec<usize>>,
    best_solution: Solution,
    worst_solution: Solution,
}

impl Diffevomaze {
    pub fn default(
        solutions: Vec<Solution>,
        problem: String,
        max_iter: usize,
        objective: fn(Vec<(usize, usize)>) -> f64,
        maze: Vec<Vec<usize>>,
    ) -> Diffevomaze {
        Diffevomaze {
            solutions,
            problem,
            max_iter,
            objective,
            best_solution: Solution::new(vec![]),
            worst_solution: Solution::new(vec![]),
            maze,
        }
    }
}

pub fn initialize_vecs(num_p: usize, m_size: usize, maze: Vec<Vec<usize>>) -> Vec<Solution> {
    let mut population = Vec::with_capacity(num_p);
    let mut rng = rand::thread_rng();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in maze.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 2 {
                start = (i, j);
            } else if cell == 4 {
                end = (i, j);
            }
        }
    }

    while population.len() < num_p {
        let num_d = rng.gen_range(1..m_size * m_size);
        let mut position = Vec::with_capacity(num_d + 2); // +2 for start and end positions
        position.push(start); // Add start position to the solution
        let mut prev = start;

        while position.len() < num_d {
            let x = rng.gen_range(0..m_size);
            let y = rng.gen_range(0..m_size);
            if maze[x][y] != 1 {
                if (x == prev.0 && (y == prev.1 + 1 || y == prev.1.checked_sub(1).unwrap_or(0)))
                    || (y == prev.1 && (x == prev.0 + 1 || x == prev.0.checked_sub(1).unwrap_or(0)))
                {
                    prev = (x, y);
                    position.push((x, y));
                }
            }
        }
        position.push(end); // Add end position to the solution
        if is_solution(maze.clone(), position.clone()) {
            let solution = Solution::new(position);
            population.push(solution);
        }
    }
    population
}

pub fn is_solution(maze: Vec<Vec<usize>>, sequence: Vec<(usize, usize)>) -> bool {
    if maze[sequence[0].0][sequence[0].1] != 2 {
        return false;
    }

    if maze[sequence[sequence.len() - 1].0][sequence[sequence.len() - 1].1] != 4 {
        return false;
    }

    let mut prev = sequence[0];
    for s in &sequence[1..] {
        let x = s.0;
        let y = s.1;

        if maze[x][y] == 1 {
            return false;
        }

        if !((x == prev.0 && (y == prev.1 + 1 || y == prev.1.checked_sub(1).unwrap_or(0)))
            || (y == prev.1 && (x == prev.0 + 1 || x == prev.0.checked_sub(1).unwrap_or(0))))
        {
            return false;
        }
        prev = *s;
    }
    true
}
