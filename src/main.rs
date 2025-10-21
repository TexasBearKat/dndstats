use rand::Rng;

fn main() {
    let stat: i32 = roll_stat();
    println!("Rolled stat: {}", stat);
}

fn verify() -> bool {
    
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