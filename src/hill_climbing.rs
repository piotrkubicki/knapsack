use std::sync::{Arc, Mutex};
use std::thread;
use crate::{Item, Search};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::seq::SliceRandom;

pub struct HillClimbing {}

impl Search for HillClimbing {}

impl HillClimbing {
    pub fn run(
        iterations: usize,
        iteration_counter: Arc<Mutex<usize>>,
        items: Arc<Vec<Item>>,
        knapsack: Arc<Mutex<Vec<Item>>>,
        max_capacity: usize
    ) -> Result<thread::JoinHandle<()>, ()> {
        Ok(thread::spawn(move || {
            let mut rng = ChaCha8Rng::seed_from_u64(101);

            for _ in 0..iterations {
                let knapsack_cpy = HillClimbing::copy_knapsack(&knapsack.lock().unwrap());
                let mut new_knapsack = HillClimbing::copy_knapsack(&knapsack_cpy);
                let _ = new_knapsack.remove(rng.gen_range(0..new_knapsack.len()));

                let mut item_ids: Vec<usize> = (0..items.len()).collect();
                item_ids.shuffle(&mut rng);

                for item_id in item_ids  {
                    let item = items.get(item_id).unwrap();

                    if new_knapsack.contains(&item) == false && HillClimbing::volume(&new_knapsack) + item.volume <= max_capacity {
                        new_knapsack.push(item.clone());
                    }
                }

                if HillClimbing::volume(&new_knapsack) <= max_capacity {
                    if HillClimbing::quality_cmp(&knapsack_cpy, &new_knapsack) == false {
                        let mut knapsack = knapsack.lock().unwrap();
                        *knapsack = new_knapsack.clone();
                    }
                }
                let mut iteration_counter = iteration_counter.lock().unwrap();
                *iteration_counter += 1;
            }
        }))
    }
}
