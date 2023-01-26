// Break up Lemire's nearly divisionless random into 4 functions
// for improved readability

extern crate rand;

// This is the 'outer', public function of this module.
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

#[inline]
fn lemire_from_seed(seed: u8, s: u8) -> Option<u16> {
    let m: u16 = seed as u16 * s as u16;
    let l: u8 = (m % 256) as u8;

    // This is a shortcut where, if l is greater than s, we know we
    // definitely have a good `m`
    if l >= s {
        return Some(m);
    }
    // calculate `floor` using a shortcut for 256 % s
    let floor: u8 = two_fifty_six_modulo(s);

    if l < floor {
        // if this seed we got generates an l that is below the floor,
        // return no m
        None
    } else {
        // but if l is at or above the floor
        // return this m so it can be used to produce a roll result
        Some(m)
    }
}

// Helper functions and comp sci shortcuts

// Faster equivalent to 256 % m
// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L393-L423
#[inline]
fn two_fifty_six_modulo(s: u8) -> u8 {
    (u8::MAX - s + 1) % s
}

// We could use a "shortcut" here where we use m >> 8 rather than m / 256
// (see: https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L291-L311)
// But we think the Rust compiler is smart enough to make this optimization for us
// I still like this long-named helper function for readability though
#[inline]
fn convert_an_m_to_a_roll_result(m: u16) -> u16 {
    m / 256
}

// Lot of unit tests!

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn two_fifty_six_modulo_shortcut_works_as_expected() {
        // You can't divide by 0, so start test input loop at 1
        for s in 1..=u8::MAX {
            let traditional_method = 256 % s as u16;
            let shortcut_method = two_fifty_six_modulo(s);

            assert_eq!(traditional_method, shortcut_method as u16);
        }
    }

    #[test]
    fn converting_from_m_to_roll_result_shortcut_works_as_expected() {
        for possible_m in 0..=u16::MAX {
            let traditional_method = possible_m / 256; // Note that the maximum value of m is 255 * 6 or 1,530
            let shortcut_method = convert_an_m_to_a_roll_result(possible_m);
            assert_eq!(traditional_method, shortcut_method);
        }
    }
}

#[cfg(test)]
mod distribution_tests {
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
    fn lemire_from_seed_function_returns_exactly_6_different_roll_results() {
        let distribution = make_distribution();
        let counts_vec = makes_counts_vector(distribution);
        assert_eq!(counts_vec.len(), 6);
    }

    #[test]
    fn lemire_from_seed_function_produces_an_even_distribution() {
        let distribution = make_distribution();
        println!("Distribution is {:?}", distribution);
        let counts_vec = makes_counts_vector(distribution);
        assert!(is_distribution_perfectly_even(counts_vec));
    }
}
