use std::sync::{Arc, Mutex};
use std::thread;
use crate::{Item, Search};
use rand::Rng;
use libm::exp;

pub struct SA {}

impl Search for SA {}

impl SA {
    pub fn run(
        iterations: usize,
        iteration_counter: Arc<Mutex<usize>>,
        items: Arc<Vec<Item>>,
        knapsack: Arc<Mutex<Vec<Item>>>,
        max_capacity: usize,
        temperature: usize
    ) -> Result<thread::JoinHandle<()>, ()> {
        Ok(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            for i in 0..iterations {
                let knapsack_cpy = SA::copy_knapsack(&knapsack.lock().unwrap());

                let mut new_knapsack = SA::copy_knapsack(&knapsack_cpy);
                let _ = new_knapsack.remove(rng.gen_range(0..new_knapsack.len()));

                let mut item_ids: Vec<usize> = (0..items.len()).collect();

                loop {
                    let item_id = item_ids.remove(rng.gen_range(0..item_ids.len()));
                    let item = items.get(item_id).unwrap();

                    if new_knapsack.contains(&item) == false && SA::volume(&new_knapsack)+ item.volume <= max_capacity {
                        new_knapsack.push(item.clone());
                    }

                    if item_ids.len() == 0 {
                        break;
                    }
                }

                let new_ks_vol = SA::volume(&new_knapsack);
                if  new_ks_vol <= max_capacity {
                    if SA::quality_cmp(&knapsack_cpy, &new_knapsack) == false {
                        let mut knapsack = knapsack.lock().unwrap();
                        *knapsack = new_knapsack.clone();
                    } else {
                        let temp = (temperature as f64) / ((i/1000)+ 1) as f64;
                        let vol_w = new_ks_vol as isize - (SA::volume(&knapsack_cpy)) as isize;
                        let val_w = SA::value(&knapsack_cpy) as isize - SA::value(&new_knapsack) as isize;
                        let diff = val_w + (vol_w / 100);
                        let m = exp(-(diff.abs() as f64) / temp);

                        if m >= rng.gen_range(0.0..1.0) {
                            let mut knapsack = knapsack.lock().unwrap();
                            *knapsack = new_knapsack.clone();
                        }
                    }
                }
                let mut iteration_counter = iteration_counter.lock().unwrap();
                *iteration_counter += 1;
            }
        }))
    }
}
