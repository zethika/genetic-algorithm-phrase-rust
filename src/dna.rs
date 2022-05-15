use random_number::random;
use rand::{
    self,
    distributions::{Distribution, Uniform},
};
use rand::rngs::ThreadRng;

#[derive(Default)]
pub struct Dna {
    genes: Vec<char>,
    pub fitness: u8
}

impl Dna {
    pub fn to_string(&self) -> String {
        self.genes.iter().collect()
    }
    pub fn build_new(genes: Vec<char>) -> Dna{
        Dna {
            fitness: 0,
            genes
        }
    }
    pub fn calculate_fitness(&mut self, target: &Vec<char>) {
        for (i, char) in target.iter().enumerate() {
            if &self.genes[i] == char {
                self.fitness += 1;
            }
        }
    }
    pub fn mutate(&mut self,rate: u8, charset: &Vec<char>, range: Uniform<u8>, rng: &ThreadRng) {
        let mut genes: Vec<char> = vec![];
        for char in self.genes.iter() {
            let random = range.sample(&mut rng.to_owned());
            if random < rate {
                genes.push(charset[random!(..=(charset.len() - 1) as u8) as usize].to_owned());
            }
            else {
                genes.push(char.to_owned())
            }
        }
        self.genes = genes;
    }
}


pub fn reproduce(parent1: &Dna, parent2: &Dna, range: Uniform<usize>, rng: &ThreadRng) -> Dna{
    let midpoint = range.sample(&mut rng.to_owned());
    let mut genes: Vec<char> = vec![];
    genes.append(&mut parent1.genes[0..midpoint].to_vec());
    genes.append(&mut parent2.genes[midpoint..parent2.genes.len()].to_vec());
    Dna {
        genes,
        fitness: 0
    }
}