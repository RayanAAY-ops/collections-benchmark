use std::collections::{HashMap, BTreeMap, HashSet, LinkedList};
use rand::{thread_rng, Rng};

fn generate_hashmap(sample_size: i32) -> HashMap<u16, u16> {
    let mut rng = thread_rng();

    // Generate random keys and values
    let keys: Vec<_> = (0..sample_size).map(|_| rng.gen::<u16>()).collect();
    let values: Vec<_> = (0..sample_size).map(|_| rng.gen::<u16>()).collect();

    // Create HashMap from keys and values
    let hashmap: HashMap<u16, u16> = keys.into_iter().zip(values.into_iter()).collect();

    hashmap // Return the created HashMap
}

fn generate_vec(sample_size: i32) -> Vec<u16> {
    let mut rng = thread_rng();

    // Generate random keys and values
    let values: Vec<u16> = (0..sample_size).map(|_| rng.gen::<u16>()).collect();

    values // Return the created HashMap
}

fn main() {
    let sample_size=10_000;
    let hashmap: HashMap<u16, u16>  = generate_hashmap(sample_size);
    let vector:Vec<u16>  = generate_vec(sample_size);

    println!("{:?}",hashmap);
    println!("{:?}",vector);


}