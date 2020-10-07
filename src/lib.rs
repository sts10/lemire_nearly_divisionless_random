extern crate rand;
use rand::prelude::*;

// Note: We can't test this function on producing a perfectly even distribution as written
#[inline]
pub fn roll_using_lemire_fast(s: u8) -> u16 {
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
    m >> 8
}

#[inline]
pub fn roll_using_gen_range(dice_size: u8) -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(0, dice_size - 1)
}

// Going to attempt to break up Lemire's into 4 or 5 functions for improved readabilityp
#[inline]
pub fn roll_using_readable_lemire(s: u8) -> u16 {
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        match lemire_from_seed(seed, s) {
            // if we get a result, that means we got a good seed and thus we got a roll
            Some(r) => return r,
            // got a bad seed and thus no roll.
            // try loop again
            None => continue,
        };
    }
}

fn lemire_from_seed(seed: u8, s: u8) -> Option<u16> {
    let m: u16 = seed as u16 * s as u16; // maximum value of m is 255 * s (if s == 6, then max of m is 1,530)
    let l: u8 = modulo_256(m); // this is a faster alternative to let l = m % 256 (see: https://doc.rust-lang.org/rust-by-example/types/cast.html)
    if l >= s {
        let roll_result = divide_by_256(m);
        return Some(roll_result);
    }
    let floor: u8 = eight_modulo(s);
    if l < floor {
        return None;
    } else {
        let roll_result = divide_by_256(m);
        return Some(roll_result);
    }
}

// comp sci shortcuts
// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L323-L358
fn modulo_256(m: u16) -> u8 {
    m as u8
}

// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L393-L423
fn eight_modulo(s: u8) -> u8 {
    (u8::MAX - s + 1) % s
}

// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L291-L311
fn divide_by_256(m: u16) -> u16 {
    m >> 8
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
            match lemire_from_seed(this_seed, 6) {
                Some(result) => all_results.push(result as usize),
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
        println!("Returning true\n{:?}", count_vec);
        true
    }

    #[test]
    fn even_distribution() {
        assert!(is_distribution_perfectly_even(make_distribution()));
    }
}
