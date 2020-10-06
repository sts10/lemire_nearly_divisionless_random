// After reading through this great explanation of something called Lemire's
// algorithm, which I learned about from:
// https://veryseriousblog.com/posts/dissecting-lemire
// and https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L294-L296

// I made an attempt at implementing it in Rust. There are likely still some
// efficiencies in the original implementation that I think I need to try
// to add in Rust.

extern crate rand;
use rand::prelude::*;

fn main() {
    // real	4.552s
    // for _n in 1..1_000_000_000 {
    //     roll_using_gen_range(6);
    // }

    // real 5.307
    // for _n in 1..1_000_000_000 {
    //     roll_using_lemire_slow(6);
    // }

    // real 4.912
    // for _n in 1..1_000_000_000 {
    //     roll_using_lemire_medium(6);
    // }

    // real 4.859s
    for _n in 1..1_000_000_000 {
        roll_using_lemire_fast(6);
    }
    println!("Done");
}

fn roll_using_lemire_slow(dice_size: usize) -> usize {
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        match lemire_slow(seed, dice_size) {
            Some(r) => return r,
            None => continue,
        };
    }
}

fn roll_using_gen_range(dice_size: u8) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0, dice_size - 1) as usize
}

fn lemire_slow(seed: u8, s: usize) -> Option<usize> {
    let rand_range_length = 256;
    let m: usize = seed as usize * s; // Note that the maximum value of m is 255 * 6 or 1,530
    let l = m % rand_range_length;
    if l >= (rand_range_length % s) {
        return Some(m >> 8);
    } else {
        None
    }
}

fn roll_using_lemire_medium(s: u8) -> u16 {
    let rand_range_length: u16 = 256; // could use `u8::MAX + 1` here, but can't imagine much of a difference?

    let seed = rand::random::<u8>(); // get a random number from 0..=255
    let m: u16 = seed as u16 * s as u16; // maximum value of m is 255 * s (if s == 6, then max of m is 1,530)
    let mut l = m % rand_range_length; // this operation is done differently in the C example
    if l < s as u16 {
        let floor: u8 = (u8::MAX - s + 1) % s;
        while l < floor as u16 {
            let seed = rand::random::<u8>(); // get a random number from 0..=255
            let m: u16 = seed as u16 * s as u16; // Note that the maximum value of m is 255 * 6 or 1,530
            l = m % rand_range_length;
        }
    }
    m >> 3
}

fn roll_using_lemire_fast(s: u8) -> u16 {
    let seed = rand::random::<u8>(); // get a random number from 0..=255
    let m: u16 = seed as u16 * s as u16; // maximum value of m is 255 * s (if s == 6, then max of m is 1,530)
    let mut l: u8 = m as u8; // this is a faster alternative to let l = m % 256 (see: https://doc.rust-lang.org/rust-by-example/types/cast.html)
    if l < s {
        let floor: u8 = (u8::MAX - s + 1) % s;
        while l < floor {
            let seed = rand::random::<u8>(); // get a random number from 0..=255
            let m: u16 = seed as u16 * s as u16; // Note that the maximum value of m is 255 * 6 or 1,530
            l = m as u8;
        }
    }
    m >> 3
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_distribution() -> HashMap<usize, usize> {
        let mut all_results: Vec<usize> = vec![];
        let lower = 0;
        let upper = 255;
        for this_seed in lower..=upper {
            match lemire_slow(this_seed, 6) {
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
