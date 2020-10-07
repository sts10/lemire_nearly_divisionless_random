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
