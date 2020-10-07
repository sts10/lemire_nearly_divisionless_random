# Notes on understanding Lemire's algorithm

Last weekend, a friend from Mastodon sent me [a really interesting blog post about something called Lemire's nearly divisonless random](https://veryseriousblog.com/posts/dissecting-lemire) written by [Colm MacCárthaigh](https://veryseriousblog.com/about). 

Apparently MacCárthaigh just wrapped up a contest "for the most readable implementations of Daniel Lemire's nearly divisionless algorithm for selecting a random number from an interval," and has awarded cash prizes to the top three.

MacCárthaigh also links to [Lemire's original blog post](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/) and his [paper](https://arxiv.org/pdf/1805.10941.pdf) from December 2018, which he defends, with caveats:

> Lemire’s accompanying paper is great and very readable, but it still takes effort and concentration to follow everything. I work on cryptography and write cryptographic code for a living and I’m not ashamed to tell you it took me about 3 readings to really get it.

Hence his contest.

> All of this makes Lemire’s algorithm a really good challenge for creating a more readable version. Ideally something that an average coder can read in one pass, understand, and agree that it’s correct. 

---

A couple things about all this intrigued me. First, I had just been working on a project that dealt with choosing random words from a long list. Second, anything shorthanded with a last name, followed by the word "algorithm" seems pretty cool, especially if it's relatively new. 

And third, I loved the idea of rewarding not workable code, put _readable_ code, or in most cases, well-written code comments. "Code readability is the most important pillar of code correctness and maintainability," MacCárthaigh writes. "When code is unreadable, it is harder to spot errors."

So I tried to understand it. 

## What we are up against

Alright so here's the C code from [Lemire's blog post](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/):

```c
uint64_t nearlydivisionless ( uint64_t s ) {
  uint64_t x = random64 () ;
  __uint128_t m = ( __uint128_t ) x * ( __uint128_t ) s;
  uint64_t l = ( uint64_t ) m;
  if (l < s) {
    uint64_t t = -s % s;
    while (l < t) {
      x = random64 () ;
      m = ( __uint128_t ) x * ( __uint128_t ) s;
      l = ( uint64_t ) m;
    }
  }
  return m >> 64;
}
```

On first blush, this made no sense to me. And even if it was in a language I can write I think I'd have still been completely lost. Thankfully, MacCárthaigh reassures:

> The second reason I chose Lemire’s algorithm is that it is impenetrable upon first reading. There are lucky few people who are so practiced and conversant in number manipulation that they can see inside of algorithms like Neo in the matrix, but I don’t mean them. To the average reader, myself included, it’s not clear what’s going on and why.

Check!

## Our starting point

As MacCárthaigh notes in his post, he used the contestants' answers to write his own explanation in [a truly wonderful and very long code comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L188). This, rather than anything written by Lemire, was my starting point to understand this thing.

I read MacCárthaigh comment through three times, picking up the tiniest bit of new knowledge each time. I got frustrated when it was immediately clear to me what was happening. But I went slow and took each section of the comment in turn.

I also started throwing some Rust code in a playground, figuring I'd attempt an example of rolling a single 6-sided die. 

### Part 1: Unfair dice

In [the first section of the comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L196-L213), MacCárthaigh walks us through a basic example of what we'll ultimately doing. 

Basically this Lemire's algorithm is a fast way of using a randomly generated number to randomly pick another number from a range. 

In the comment's example, we've got a random number that's either 0, 1, 2, 3, 4, 5, 6, or 7. And what we want is "to use that to generate a number in the set 0, 1, 2."

The first thing we'll try is the modulus or remainder operator (`%`). MacCárthaigh observes: "A naive way to to this is to simply use x % 3. % is the modulus or remainder * operator and it returns the remainder left over from x/3." Why is it "naive"? We'll find out!

For my 6-sided die example, I would have used 0 through 7 for my large, initial random number (which I'm going to call a "seed"), but Rust doesn't easily support 3-bit numbers. 

The smallest bit number Rust easily supports is 8 bits (`u8`), which covers a range of 0 to 255. So I used that. In Rust I did this with the rand crate with `let seed_up_to_255 = rand::random::<u8>();`

Given the way bits and computers work, I think computers are very fast at generating random numbers if the range you want it is the length of 2^x. Since 256 = 2^8, generating this initial random number happens very quickly. The problem Lemire's code address is taking one of these "fast" random numbers and quickly adapting them to _any_ given range of numbers.

For the dice roll result I decided to start at 0, so it's 0 through 5. 

```rust
extern crate rand;
use rand::prelude::*;

fn naive_modulus_dice_roll() {
    let seed_up_to_255 = rand::random::<u8>(); // get a random number from 0..=255
    let dice_roll = roll(seed_up_to_255);
    println!("Naive dice roll of {}", dice_roll);
}

fn roll(seed: u8) -> u8 {
    seed % 6
}
```

As MacCárthaigh [shows more clearly in his comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L205-L213), the problem comes at the end (or the higher end of our potential seed values), when our rotation of 0, 1, 2 abruptly ends at 1, rather than cleanly ending wtih a 2. 

I explored this problem in my Rust example using `assert_eq!` statements. Remember, our seed values can range from 0 to 255. Let's see what happens for seeds of 249 and up:

```rust
    // a seed of 249 yields a dice roll of 3
    assert_eq!(roll(249), 3);
    // a seed of 251 yields a dice roll of 4 ... all good so far
    assert_eq!(roll(250), 4);
    assert_eq!(roll(251), 5);
    assert_eq!(roll(252), 0);
    assert_eq!(roll(253), 1);
    assert_eq!(roll(254), 2);
    assert_eq!(roll(255), 3);
    // But the seed _can't_ be 256 or 257 (too high for u8),
    // so these last 4 seed values that yield dice rolls of 0, 1, 2, and 3 are _extra_
    // In other words the whole process favors rolls of 0 to 3 at the expense of results 4 and 5.
```

## Code I'm pulling from

```rust
extern crate rand; // 0.7.3
fn main() {
    // learning_about_simplest_dice_roll();
    // rejection_fix();
    // lemire_unfair();
    let random_dice_roll: usize;
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        match lemire_slow(seed) {
            Some(r) => {
                random_dice_roll = r;
                break;
            }
            None => continue,
        }
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

fn lemire_1_slow_basic(seed: u8) -> Option<usize> {
    // loop {
    // let seed = rand::random::<u8>(); // get a random number from 0..=255
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
    let l = m % 256;
    if l >= (256 % 6) {
        return Some(m >> 8);
    } else {
        None
    }
}
```

I then broke this initial attempt at Lemire's into two functions: 

```rust
fn roll_using_lemire_slow(dice_size: usize) -> usize {
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        match lemire_slow(seed, dice_size) {
            Some(r) => return r,
            None => continue,
        };
    }
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
```

We can test this `lemire_slow` function for equal distribution with the following:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_distribution() -> HashMap<usize, usize> {
        let mut all_results: Vec<usize> = vec![];
        let lower = 0;
        let upper = 255;
        for this_seed in lower..=upper {
            match lemire_slow(this_seed) {
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
        println!("Returning true\n{:?}", count_vec);
        true
    }

    #[test]
    fn even_distribution() {
        assert!(is_distribution_perfectly_even(make_distribution()));
    }
}
```


Early attempt at speeding up lemire_slow:

```rust
fn lemire_medium_test(seed: u8) -> usize {
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
    let l = m % 256;
    if l <= 6 {
        let floor = 8 % 6;

        return m >> 8;
    }
}
```

Next attempt at Lemire: 

```rust
// Can't test for even distribution as written
fn roll_using_lemire_medium(seed: u8, s: u8) -> u16 {
    // let seed = rand::random::<u8>(); // get a random number from 0..=255
    let rand_range_length: u16 = 256; // could use `u8::MAX + 1` here, but can't imagine much of a difference?

    let m: u16 = seed as u16 * s as u16; // maximum value of m is 255 * s (if s == 6, then max of m is 1,530)
    let mut l = m % rand_range_length; // this operation is done differently in the C example

    if l < s as u16 {
        let floor = rand_range_length % s as u16;
        while l < floor {
            let seed = rand::random::<u8>(); // get a random number from 0..=255
            let m: u16 = seed as u16 * s as u16; // Note that the maximum value of m is 255 * 6 or 1,530
            l = m % rand_range_length;
        }
    }
    m >> 8
}
```

## Fast Lemire

```rust
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
```

## Writing benchmark tests using the Criterion crate: 


Not going to lie, did a lot of copy and pasting from: 
https://bheisler.github.io/criterion.rs/book/getting_started.html

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lemire::roll_using_gen_range;
use lemire::roll_using_lemire_fast;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lemire fast 6", |b| {
        b.iter(|| roll_using_lemire_fast(black_box(6)))
    });

    c.bench_function("rand 6", |b| b.iter(|| roll_using_gen_range(black_box(6))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```


Initial Criterion benchmark results: Lemire benchmarks a few nanoseconds faster!

```text
lemire fast 6           time:   [5.8207 ns 5.8747 ns 5.9328 ns]                           
                        change: [+4.5523% +5.6439% +6.7200%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

rand 6                  time:   [8.1261 ns 8.1815 ns 8.2392 ns]                    
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```



## Appendix

### Some URLs


https://veryseriousblog.com/posts/dissecting-lemire
and https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L294-L296

original blog post: https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/
paper: https://arxiv.org/pdf/1805.10941.pdf
