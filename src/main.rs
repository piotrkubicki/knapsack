use std::{thread, time};
use std::sync::{Arc, Mutex};
use hill_climbing::HillClimbing;
use sa::SA;
use rand::Rng;

mod hill_climbing;
mod sa;


#[derive(Clone)]
pub struct Item {
    pub id: usize,
    pub value: usize,
    pub volume: usize,
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id
    }
}

pub trait Search {
    fn value(items: &Vec<Item>) -> usize {
        let mut total = 0;

        for item in items.iter() {
            total += item.value;
        }

        total
    }

    fn quality_cmp(l_items: &Vec<Item>, r_items: &Vec<Item>) -> bool {
        let l_value = Self::value(l_items);
        let r_value = Self::value(r_items);

        if r_value > l_value {
            return false;
        }

        let l_volume = Self::volume(l_items);
        let r_volume = Self::volume(r_items);

        if r_value == l_value && r_volume < l_volume {
            return false;
        }

        true
    }

    fn volume(items: &Vec<Item>) -> usize {
        let mut total = 0;

        for item in items.iter() {
            total += item.volume;
        }

        total
    }

    fn copy_knapsack(knapsack: &Vec<Item>) -> Vec<Item> {
        knapsack.clone()
    }
}

fn generate_random_item(id: usize) -> Item {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(17..=23);
    let volume = rng.gen_range(11..=17);
    Item { id, value, volume }
}

fn main() {
    const MAX_CAPACITY: usize = 400;
    const ITERATIONS: usize = 600000;
    let mut items = Vec::new();

    for i in 0..140 {
        let item = generate_random_item(i);
        items.push(item);
    }
    let items = Arc::new(items);
    let mut knapsack: Vec<Item> = vec![];

    for item in items.iter() {
        if knapsack.contains(&item) == false && (HillClimbing::volume(&knapsack) + item.volume) < MAX_CAPACITY {
            knapsack.push(item.clone());
        }
    }

    println!("Initial value: {} capacity: {}\n\n", HillClimbing::value(&knapsack), HillClimbing::volume(&knapsack));

    let hc_knapsack = Arc::new(Mutex::new(knapsack.clone()));
    let hc_iteration_counter = Arc::new(Mutex::new(0));
    let hill_climbing_join = HillClimbing::run(ITERATIONS, hc_iteration_counter.clone(), items.clone(), hc_knapsack.clone(), MAX_CAPACITY).unwrap();

    let sa_knapsack = Arc::new(Mutex::new(knapsack.clone()));
    let sa_iteration_counter = Arc::new(Mutex::new(0));
    let sa_join = SA::run(ITERATIONS, sa_iteration_counter.clone(), items.clone(), sa_knapsack.clone(), MAX_CAPACITY, 100).unwrap();

    while hill_climbing_join.is_finished() == false || sa_join.is_finished() == false {
        thread::sleep(time::Duration::from_millis(100));
        let hc_knapsack = hc_knapsack.lock().unwrap();
        println!(
            "\x1b[2FHC iteration: {} Current best: {} Capacity: {}                 ",
            hc_iteration_counter.lock().unwrap(),
            HillClimbing::value(&hc_knapsack),
            HillClimbing::volume(&hc_knapsack)
        );
        let sa_knapsack = sa_knapsack.lock().unwrap();
        println!(
            "FS iteration: {} Current best: {} Capacity: {}                 ",
            sa_iteration_counter.lock().unwrap(),
            SA::value(&sa_knapsack),
            SA::volume(&sa_knapsack)
        );
    }

}