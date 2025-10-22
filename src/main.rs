use rand::Rng;
use std::time::{Duration, Instant};
use std::io;
// branch test

fn main() {
    let mut fits_specs: bool = false;
    let mut acc: u32 = 0;
    let mut stat_block: Vec<i32> = vec![];
    let mut random_seed = rand::thread_rng()
    
    // user input kinda goes hard

    let inputs: Vec<i32> = take_user_input();

    let count: i32 = inputs[0];
    let target: i32 = inputs[1];
    let least: i32 = inputs[2];

    // benchmarking!!
    let mut start_time: Instant = Instant::now(); 
    

    while !fits_specs {        
        acc += 1;

        stat_block = create_stat_set(&random_seed);
        fits_specs = verify(&stat_block, count, target, least); 

        if acc % 1000000 == 0 {
            let elapsed_time: Duration = start_time.elapsed();
            let ops: f64 = (1_000_000.0 / elapsed_time.as_millis() as f64) * 1000 as f64;

            println!("1 million iterations in: {:?}\n{} operations per second\n{} total operations\n", elapsed_time, ops, acc);

            start_time = Instant::now();
        }   
    }
    println!("{:?}, Iterations: {}", stat_block, acc);
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
    
    amount >= count
}

fn roll_stat(rng: &mut impl Rng) -> i32 {
    let mut rolls = [0; 4];
    
    for i in 0..4 {
        rolls[i] = rng.gen_range(1..7)
    }
    
    let min = *rolls.iter().min().unwrap();
    rolls.iter().sum::<i32>() - min
}

fn create_stat_set(rng: &mut impl Rng) -> Vec<i32> {
    (0..6).map(|_| roll_stat(rng)).collect()
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

fn take_user_input() -> Vec<i32> {
    let mut inputs: Vec<i32> = vec![];

    let prompts: Vec<&str> = vec!["Count?", "Target?", "Least?"];

    for x in prompts.iter() {
        let mut input: String = String::new();

        println!("{}", x);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => inputs.push(num),
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                return vec![0]
            }
        }
    }
    inputs
}

// hehehehehehehehehe