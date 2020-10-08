#[cfg(test)]
mod tests {
    use lemire::convert_an_m_to_a_roll_result;
    use lemire::lemire_from_seed;
    use lemire::modulo_256;
    use lemire::two_fifty_six_modulo;
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

    // Test the shortcut helper functions

    #[test]
    fn modulo_256_shortcut_works_as_expected() {
        for possible_m in 0..=u16::MAX {
            let traditional_method = possible_m % 256; // Note that the maximum value of m is 255 * 6 or 1,530
            let shortcut_method = modulo_256(possible_m);
            assert_eq!(traditional_method, shortcut_method as u16);
        }
    }

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
