use colored::*;

#[derive(Debug,Clone)]
pub struct Particle{
    pub position: Vec<f64>,
    pub avalue: f64
}
#[allow(dead_code)]
impl Particle{
    pub fn new(position: Vec<f64>) -> Particle{
        Particle{
            position,
            avalue: 0.0
        }
    }

    pub fn print(&self) {
        println!(
            "{}",
            format!("(position:{:?},a_value:{:?})", self.position, self.avalue).blue()
        );
    }

}

