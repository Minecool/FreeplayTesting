use crate::seeded_random::SeededRandom;
use std::time::Instant;
mod constants;
mod seeded_random;

pub fn calculate(freeplay_round_seed: i64, start_round: i32, end_round: i32) {
    // println!("Seed: {}", freeplay_round_seed);
    for current_round in start_round..=end_round {
        // println!("Round: {}", current_round);

        let mut list: [u16; constants::LIST_SIZE] = [0; constants::LIST_SIZE];
        let current_seed: i64 = freeplay_round_seed + current_round as i64;

        for i in 0..constants::LIST_SIZE {
            list[i] = i as u16;
        }

        shuffle_in_place(list, current_seed as i32);
        // println!("{:?}", list);
        print_rounds(current_seed, current_round, list);
    }
}

fn shuffle_in_place(mut arr: [u16; constants::LIST_SIZE], seed: i32) {
    let mut rng = SeededRandom::new(seed as i64);

    for i in 0..arr.len() {
        let j = rng.range(i as i32, arr.len() as i32);
        arr.swap(i, j as usize);
    }
    // println!("{:?}", arr);
}

fn print_rounds(current_seed: i64, current_round: i32, arr: [u16; constants::LIST_SIZE]) {
    let mut rng: SeededRandom = SeededRandom::new(current_seed);
    let mut budget = constants::BASE_BUDGETS[current_round as usize];
    if 1 < current_round {
        budget -= (rng.next_float() - 0.5) * budget;
    }

    for &i in &arr {
        let i = i as usize;
        if &constants::LOWER_BOUNDS[i] <= &current_round
            && &current_round <= &constants::UPPER_BOUNDS[i]
            && &constants::SCORES[i] <= &budget
        {
            budget -= constants::SCORES[i];
            // println!("{}", constants::ROUND_NAMES[i]);
        }
    }
}

fn main() {
    let now = Instant::now();

    let start_round: i32 = 141;
    let end_round: i32 = 999;
    let start_seed: i32 = 1;
    let end_seed: i32 = 1000;

    for seed in start_seed..=end_seed {
        calculate(seed as i64, start_round, end_round);
    }
    // (start_seed..end_seed).into_par_iter().for_each(|seed| {
    //     calculate(seed as i64, start_round, end_round);
    // });

    println!("{}ms", now.elapsed().as_millis());
}
