use rand::{
    self,
    distributions::{Distribution, Uniform},
}; // 0.8.0
use crate::dna::{Dna, reproduce};
use crate::random;
use rayon::prelude::*;

pub struct Population {
    population_size: u16,
    mutation_rate: u8,
    target: Vec<char>,
    pub highest_fitness: u8,
    pub highest_fitness_string: String,
    pub population: Vec<Dna>,
    pub generation: u32,
    pub total_fitness: u32,
    pool: Vec<u16>,
}

impl Population {
    pub fn build_new(population_size: u16, mutation_rate: u8, target: Vec<char>, charset: &Vec<char>) -> Population {
        let initial_population = (0..population_size).map(|_x|Dna::build_new(random::generate_random_string(target.len() as u32,&charset))).collect();
        Population {
            population_size,
            mutation_rate,
            target,
            highest_fitness: 0,
            highest_fitness_string: "".to_string(),
            population: initial_population,
            generation: 0,
            total_fitness: 0,
            pool: vec![]
        }
    }

    pub fn evaluate_generation(&mut self) {
        self.total_fitness = 0;
        self.highest_fitness = 0;
        self.highest_fitness_string = "".to_string();
        for dna in &mut self.population {
            dna.calculate_fitness(&self.target);
            self.total_fitness += dna.fitness as u32;

            if dna.fitness > self.highest_fitness {
                self.highest_fitness = dna.fitness;
                self.highest_fitness_string = dna.to_string();
            }
        }

        let mut pool = vec![];
        for (i,dna) in self.population.iter().enumerate() {
            if dna.fitness != 0 {
                let mut vec: Vec<u16> = vec![i as u16; map_value_to_range(dna.fitness as i32, 0, self.highest_fitness as i32, 0, 100) as usize];
                pool.append(&mut vec)
            }
        }
        self.pool = pool;
    }

    pub fn move_to_next_generation(&mut self, charset: &Vec<char>) {

        let population_range = Uniform::new_inclusive(0, self.pool.len()-1);
        let gene_range = Uniform::new_inclusive(1, self.target.len()-1);
        let percentage_range: Uniform<u8> = Uniform::new_inclusive(0, 100);

        let result: Vec<Dna> = vec![0;self.population_size as usize].par_iter().map(|_i| create_child(&self.population,&self.pool,self.mutation_rate,charset,population_range,gene_range,percentage_range)).collect();
        self.population = result;
        self.generation += 1;
    }
}
fn create_child(population: &Vec<Dna>, pool: &Vec<u16>, mutation_rate: u8, charset: &Vec<char>, population_range: Uniform<usize>,gene_range: Uniform<usize>,percentage_range: Uniform<u8>) -> Dna{
    let mut rng = rand::thread_rng();
    let random1 = population_range.sample(&mut rng);
    let random2 = population_range.sample(&mut rng);
    let mut child = reproduce(&population[pool[random1] as usize],&population[pool[random2] as usize], gene_range, &rng);
    child.mutate(mutation_rate,charset,percentage_range,&rng);
    return child
}


fn map_value_to_range(value: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    return (value - x1) * (y2 - x2) / (y1 - x1) + x2;
}


