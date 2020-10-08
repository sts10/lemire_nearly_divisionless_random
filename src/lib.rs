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
            // if we get a an m value back, that means we had a seed that produced a "good" m
            // meaning an m we can use to generate a roll result
            Some(m) => return convert_an_m_to_a_roll_result(m),
            // If we're here, we got a bad seed and thus a bad m. No roll result
            // returned by lemire_from_seed function.
            // So let's go back to the top of the `loop`.
            None => continue,
        };
    }
}

fn lemire_from_seed(seed: u8, s: u8) -> Option<u16> {
    let m: u16 = seed as u16 * s as u16; // maximum value of m is 255 * s (if s == 6, then max of m is 1,530)
    let l: u8 = modulo_256(m); // this is a faster alternative to let l = m % 256 (see: https://doc.rust-lang.org/rust-by-example/types/cast.html)
    if l >= s {
        return Some(m);
    }
    let floor: u8 = eight_modulo(s);
    if l < floor {
        return None;
    } else {
        return Some(m);
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
// This is the same as dividing by 256, which, given our constants, is how we convert an m to a
// result
fn convert_an_m_to_a_roll_result(m: u16) -> u16 {
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
                Some(m) => {
                    let roll_result = convert_an_m_to_a_roll_result(m) as usize;
                    all_results.push(roll_result);
                }
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

    fn makes_counts_vector(counts_hashmap: HashMap<usize, usize>) -> Vec<(usize, usize)> {
        counts_hashmap.into_iter().collect()
    }

    fn is_distribution_perfectly_even(count_vec: Vec<(usize, usize)>) -> bool {
        let first_count = count_vec[0].1;
        for result in &count_vec {
            if result.1 != first_count {
                return false;
            }
        }
        true
    }

    #[test]
    fn has_exactly_6_different_roll_results() {
        let distribution = make_distribution();
        let counts_vec = makes_counts_vector(distribution);
        assert_eq!(counts_vec.len(), 6);
    }

    #[test]
    fn even_distribution() {
        let distribution = make_distribution();
        println!("Distribution is {:?}", distribution);
        let counts_vec = makes_counts_vector(distribution);
        assert!(is_distribution_perfectly_even(counts_vec));
    }
}
