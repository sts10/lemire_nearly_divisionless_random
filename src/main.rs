extern crate rand; // 0.7.3
fn main() {
    // Attempting my own explanation of Lemire's algorithm using Rust
    // https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L187
    //  https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/
    //  https://arxiv.org/pdf/1805.10941.pdf
    //
    //  Here's where I'm stuck at this point:
    //  https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a2533f5f5e64e451d9c8910e4ce048a9

    // learning_about_simplest_dice_roll();
    // rejection_fix();
    // lemire_unfair();
    // println!("Lemiere slow give: {}", lemire_slow());
    // let seed_to_test = 39;
    for seed_to_test in 0..=255 {
        println!(
            "Lemiere slow for {} gives: {:?}",
            seed_to_test,
            lemire_slow_test(seed_to_test)
        );
    }
}

fn roll(seed: u8) -> u8 {
    seed % 6
}

fn learning_about_simplest_dice_roll() {
    for _n in 0..3 {
        let seed_up_to_255 = rand::random::<u8>(); // get a random number from 0..=255
        let dice_roll = roll(seed_up_to_255);
        println!("Bad dice roll of {}", dice_roll);
    }
    // the problem here is what happens at the very high-end of seed values.
    assert_eq!(roll(249), 3);
    assert_eq!(roll(250), 4);
    assert_eq!(roll(251), 5);
    assert_eq!(roll(252), 0);
    assert_eq!(roll(253), 1);
    assert_eq!(roll(254), 2);
    assert_eq!(roll(255), 3);
    // But the seed _can't_ be 256 or 257 (too  hihg for u8),
    // so these last values of 0, 1, 2, and 3 for the die roll are extra
    // thus the whole process favors rolls of 0 to 3 at the expense
    // of results 4 and 5.
}

fn rejection_fix() {
    // One solution to this problem is to call a "do over" if the seed
    // is 252, 253, 254, or 255

    // We could hard-code something like
    // while seed < 252

    // but let's write a formula to find the 252 number, given the maximum
    // of the random number seed and the length of the range of random
    // number we actually want:
    let ceiling = 255 - (255 % 6); // is 252
    assert_eq!(ceiling, 252);

    // Now we can do ...
    for _n in 0..350 {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        if seed < ceiling {
            println!("Rejection Method dice roll: {}", roll(seed));
        } else {
            println!("Got a bad seed of {}! Getting a new seed.", seed);
            continue;
        }
    }
    // the bummer here is that in 4 our of 256 seeds, we have to do a do-over,
    // which isn't ideal for efficiency.
}

fn lemire_unfair() {
    let seed = rand::random::<u8>(); // get a random number from 0..=255

    // Kind of blidnly trusting the explanation of Lemire's algorithm,
    // we're going to calcualte m like this:
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530

    // So m is a random number, with values that are multiples of 6:
    // 0, 6, 12, 18, 24, 30, etc.  up to 1,530

    // Note that we can easily get a dice roll (though not a fair one) from m by dividing it by 256
    for _n in 0..10 {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
        let example_roll = m / 256;
        // println!("Example roll using m and division: {}", example_roll);
    }

    // apparently thanks to the nature of u8 integers, dividing by 256 can also be done be using a
    // "bit shift" of 8.
    // In Rust, this is represented by m >> 8

    for _n in 0..10 {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
        let example_roll = m >> 8;
        println!(
            "Using seed {} and an m of {}, an example roll using a bit shift: {}",
            seed, m, example_roll
        );
        assert_eq!(m >> 8, m / 256);
    }

    // But either way we slice m, it's still unfair in a similar way that our initial roll function is
    // unfair.
    //
    // For seeds from 0 to 42 (43 seed values), we get a dice roll of 0
    assert_eq!((42 * 6) >> 8, 0);
    // For seeds from 43 to 85 (43 seed values), we get a dice roll of 1
    assert_eq!((43 * 6) >> 8, 1);
    assert_eq!((85 * 6) >> 8, 1);
    // For seeds from 86 to 127 (42 seed values), we get a dice roll of 2
    assert_eq!((86 * 6) >> 8, 2);
    assert_eq!((127 * 6) >> 8, 2);
    // For seeds from 128 to 170 (43), we get a dice roll of 3
    assert_eq!((128 * 6) >> 8, 3);
    assert_eq!((170 * 6) >> 8, 3);
    // For seeds from 171 to 213 (43), we get a dice roll of 4
    // For seeds from 214 to 255 (42), we get a dice roll of 5
}

fn lemire_slow() -> usize {
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
        let l = m % 8;
        if l < (8 % 6) {
            return m >> 8;
        }
    }
}

fn lemire_slow_test(seed: u8) -> Option<usize> {
    // loop {
    // let seed = rand::random::<u8>(); // get a random number from 0..=255
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
    let l = m % 256;
    println!("m is {}; l in {}", m, l);
    if l > (256 % 6) {
        return Some(m >> 8);
    } else {
        None
    }
    // Seeds 1 to 42    give dice 0 (42)
    // Seeds 44 to 85   give dice 1 (42)
    // Seeds 87 to 127  give dice 2 (41)
    // Seeds 129 to 170 give dice 3 (42)
    // Seeds 172 to 213 give dice 4 (42)
    // Seeds 215 to 255 give dice 5 (41)
}

// fn lemire_medium_test(seed: u8) -> usize {
//     let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
//     let l = m % 8;
//     if l < 6 {
//         let floor = 8 % 6;

//         return m >> 8;
//     }

// }
//

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_distribution() -> HashMap<usize, usize> {
        let mut all_results: Vec<usize> = vec![];
        let lower = 0;
        let upper = 255;
        for this_seed in lower..=upper {
            match lemire_slow_test(this_seed) {
                Some(result) => all_results.push(result),
                None => continue,
            }
        }

        let mut counts_hashmap: HashMap<usize, usize> = HashMap::new();
        for result in all_results {
            counts_hashmap
                .entry(result)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        counts_hashmap
    }

    fn is_distribution_perfectly_even(counts_hashmap: HashMap<usize, usize>) -> bool {
        let count_vec: Vec<(&usize, &usize)> = counts_hashmap.iter().collect();
        let first_count = count_vec[0].1;
        for result in &count_vec {
            if result.1 != first_count {
                println!("Returning false\n{:?}", count_vec);
                return false;
            }
        }
        true
    }

    #[test]
    fn even_distribution() {
        assert!(is_distribution_perfectly_even(make_distribution()));
    }
}
