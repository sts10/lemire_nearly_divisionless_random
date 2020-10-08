extern crate rand;
use rand::prelude::*;

// My finished implementation of Lemire's divisionless random
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

// A stand-in of how the Rand crate rolls a six-sided die, for benchmarking purposes
#[inline]
pub fn roll_using_gen_range(dice_size: u8) -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(0, dice_size - 1)
}

// Break up Lemire's divisionless random into 4 or 5 functions for improved readabilityp
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

pub fn lemire_from_seed(seed: u8, s: u8) -> Option<u16> {
    let m: u16 = seed as u16 * s as u16;

    // use a shortcut for m % 256 to calculate l faster
    let l: u8 = modulo_256(m);
    if l >= s {
        return Some(m);
    }
    // calculate `floor` using a shortcut for 256 % s
    let floor: u8 = two_fifty_six_modulo(s);

    if l < floor {
        // if this seed we got generates an l that is below the floor,
        // return no m
        return None;
    } else {
        // but if l is at or above the floor
        // return this m so it can be used to produce a roll result
        return Some(m);
    }
}

// comp sci shortcuts
// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L323-L358
pub fn modulo_256(m: u16) -> u8 {
    m as u8
}

// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L393-L423
pub fn two_fifty_six_modulo(s: u8) -> u8 {
    (u8::MAX - s + 1) % s
}

// https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L291-L311
// This is the same as dividing by 256, which, given our constants, is how we convert an m to a
// result
pub fn convert_an_m_to_a_roll_result(m: u16) -> u16 {
    m >> 8
}
