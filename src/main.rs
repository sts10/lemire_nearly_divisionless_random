extern crate rand; // 0.7.3
fn main() {
    // Attempting my own explanation of Lemire's algorithm using Rust
    // https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L187
    //  https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/
    //  https://arxiv.org/pdf/1805.10941.pdf

    // learning_about_simplest_dice_roll();
    // rejection_fix();
    lemire();
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

fn lemire() {
    let seed = rand::random::<u8>(); // get a random number from 0..=255

    // Kind of blidnly trusting the explanation of Lemire's algorithm,
    // we're going to calcualte m like this:
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530

    // So m is a random number, with values that are multiples of 6:
    // 0, 6, 12, 18, 24, 30, etc.  up to 1,530

    // Note that we can easily get a dice roll (though not a fair one) from m by dividing it by 255
    for _n in 0..10 {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
        let example_roll = m / 255;
        println!("Example roll using m and division: {}", example_roll);
    }

    // apparently thanks to the nature of u8 integers, dividing by 255 can also be done be using a
    // "bit shift" of 8.
    // In Rust, this is represented by m >> 8

    for _n in 0..10 {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
        let example_roll = m >> 8;
        println!("Example roll using m and a bit shift: {}", example_roll);
    }

    // But either way we slice m, it's still unfair in the same way our initial roll function is
    // unfair.
    //
}
