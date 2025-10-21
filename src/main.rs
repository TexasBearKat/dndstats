use rand::Rng;
use std::time::{Duration, Instant};

fn main() {
    let mut fits_specs: bool = false;
    let mut acc: u32 = 0;

    // benchmarking!!
    let mut start_time: Instant = Instant::now(); 

    while !fits_specs {        
        acc += 1;

        let stat_block = create_stat_set();
        fits_specs = verify(&stat_block, 5, 18, 16); 

        if fits_specs {
            println!("{:?}, Iterations: {}", stat_block, acc);
        }

        if acc % 1000000 == 0 {
            let elapsed_time: Duration = start_time.elapsed();
            let ops: f64 = (1.0 / elapsed_time.as_millis() as f64) * 1000000000 as f64;

            println!("1 million iterations in: {:?}\n{} operations per second\n", elapsed_time, ops);

            start_time = Instant::now();
        }
            
        
    }
}

fn verify(vec: &Vec<i32>, count: i32, target: i32, least: i32) -> bool {
    if vec.is_empty() {
        return false;
    }
    
    let mut amount: i32 = 0;
    
    for &x in vec.iter() {
        if x < least {
            return false;
        }
        
        if x >= target {
            amount += 1;
        }
    }
    
    if amount >= count {
        return true;
    }
    false
}

fn roll_stat() -> i32 {
    let mut rolls: Vec<i32> = Vec::new();
    let mut rng = rand::rng();

    while rolls.len() < 4 {
        rolls.push(rng.random_range(1..=6));
    }

    rolls = drop_lowest(rolls);
    rolls.iter().sum()
}

fn create_stat_set() -> Vec<i32> {
    let mut set: Vec<i32> = vec![];

    while set.len() < 6 {
        set.push(roll_stat());
    }

    set
}



fn drop_lowest(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.is_empty() {
        return vec;
    }

    let (mut min_index, mut min_value) = (0, vec[0]);
    for (i, &val) in vec.iter().enumerate() {
        if val < min_value {
            min_value = val;
            min_index = i;
        }
    }

    vec.remove(min_index);
    vec
}