extern crate core;

use crate::population::Population;
use std::time::{Instant};
mod dna;
mod population;
mod random;

const POPULATION_SIZE: u16 = 200;
const MUTATION_RATE: u8 = 1;
const TARGET: &str = "To be or not to be.";

fn main() {
    let full = Instant::now();
    let charset = random::generate_char_vec();
    let target: Vec<char> = TARGET.chars().collect();
    let target_len = target.len();
    let mut population = Population::build_new(POPULATION_SIZE, MUTATION_RATE, target,&charset);

    loop
    {
        //println!("Generation: {}", population.generation);

        //println!("  Eval");
        //let evaluate = Instant::now();
        population.evaluate_generation();
        //let eval_duration = evaluate.elapsed();
        //println!("      Took: {:?}", eval_duration);
        if population.highest_fitness as usize == target_len {
            break;
        }

        //println!("  Next");
        //let next = Instant::now();
        population.move_to_next_generation(&charset);
        //let next_duration = next.elapsed();

        //println!("      Took: {:?}", next_duration);
    }

    let next_duration = full.elapsed();
    let pr_milli: f32 = population.generation as f32/next_duration.as_millis() as f32;
    println!("Finished in {} generations at {} generations pr second!",population.generation,pr_milli*1000 as f32);
}

