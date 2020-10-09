extern crate rand;

// An implementation of Lemire's  nearly divisionless random in Rust using
// a number of computational "shortcuts"
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

// Here's a more simple, traditional version of this type of function,
// just using modulo and rejection
#[inline]
pub fn roll_using_traditional_rejection_method(s: u8) -> u8 {
    let ceiling = 255 - (255 % 6); // is 252

    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        if seed < ceiling {
            // Got a good seed, so we'll make it a dice roll and return it
            return seed % s;
        } else {
            // Got a bad seed (too high)!
            // Return to the top of this loop to get a new seed
            continue;
        }
    }
}
