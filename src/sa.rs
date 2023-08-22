use std::sync::{Arc, Mutex};
use std::thread;
use crate::{Item, Search};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::seq::SliceRandom;
use libm::exp;
use crate::Writter;
use chrono::prelude::*;

pub struct SA {}

impl Search for SA {}

impl SA {
    pub fn run(
        writter: Box<dyn Writter + Send>,
        iterations: usize,
        iteration_counter: Arc<Mutex<usize>>,
        items: Arc<Vec<Item>>,
        knapsack: Arc<Mutex<Vec<Item>>>,
        max_capacity: usize,
        temperature: usize
    ) -> Result<thread::JoinHandle<()>, ()> {
        let filename = format!("{}_{}_{}_{}.csv", "sa", iterations, temperature, Utc::now().timestamp());
        let data = vec![
            filename.clone(),
            "id".to_string(),
            "iteration".to_string(),
            "temperature".to_string(),
            "weight".to_string(),
            "value".to_string()
        ];
        writter.write(data);
        Ok(thread::spawn(move || {
            let mut rng = ChaCha8Rng::seed_from_u64(100);

            for i in 0..iterations {
                let knapsack_cpy = SA::copy_knapsack(&knapsack.lock().unwrap());

                let mut new_knapsack = SA::copy_knapsack(&knapsack_cpy);
                let _ = new_knapsack.remove(rng.gen_range(0..new_knapsack.len()));

                let mut item_ids: Vec<usize> = (0..items.len()).collect();
                item_ids.shuffle(&mut rng);

                for item_id in item_ids {
                    let item = items.get(item_id).unwrap();

                    if new_knapsack.contains(&item) == false && SA::volume(&new_knapsack)+ item.volume <= max_capacity {
                        new_knapsack.push(item.clone());
                    }
                }

                let new_ks_vol = SA::volume(&new_knapsack);
                let temp = (temperature as f64) / ((i/1000)+ 1) as f64;
                let vol_w = new_ks_vol as isize - (SA::volume(&knapsack_cpy)) as isize;
                let val_w = SA::value(&knapsack_cpy) as isize - SA::value(&new_knapsack) as isize;
                let diff = val_w + (vol_w / 100);
                let m = exp(-(diff.abs() as f64) / temp);

                if new_ks_vol <= max_capacity {
                    if SA::quality_cmp(&knapsack_cpy, &new_knapsack) == false {
                        let mut knapsack = knapsack.lock().unwrap();
                        *knapsack = new_knapsack.clone();
                    } else {
                        if m >= rng.gen_range(0.0..1.0) {
                            let mut knapsack = knapsack.lock().unwrap();
                            *knapsack = new_knapsack.clone();
                        }
                    }
                }
                let mut iteration_counter = iteration_counter.lock().unwrap();
                if *iteration_counter % 100 == 0 {
                    let data: Vec<String>;
                    {
                        data = vec![
                            filename.clone(),
                            "SA".to_string(),
                            iteration_counter.to_string(),
                            temp.to_string(),
                            m.to_string(),
                            SA::value(&knapsack.lock().unwrap()).to_string()
                        ];
                    }
                    let _ = writter.write(data);
                }
                *iteration_counter += 1;
            }
        }))
    }
}
