# An implementation of Lemire's nearly divisionless random in Rust

TL;DR As a learning experience, I tried write my own implementation of [Lemire's nearly divisionless random](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/) in Rust. Currently it's limited to 8-bit integers. 

Disclaimer: Not a cryptographer or even a programmer by trade. 

---

# My notes on understanding Lemire's algorithm

Last weekend, a mutual on Mastodon sent me [a really interesting blog post about something called Lemire's nearly divisonless random](https://veryseriousblog.com/posts/dissecting-lemire) written by [Colm MacCárthaigh](https://veryseriousblog.com/about). 

Apparently MacCárthaigh just wrapped up a contest "for the most readable implementations of Daniel Lemire's nearly divisionless algorithm for selecting a random number from an interval," and has awarded cash prizes to the top three.

MacCárthaigh also links to [Lemire's original blog post](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/) and his [paper](https://arxiv.org/pdf/1805.10941.pdf) from December 2018, which he defends, with caveats:

> Lemire’s accompanying paper is great and very readable, but it still takes effort and concentration to follow everything. I work on cryptography and write cryptographic code for a living and I’m not ashamed to tell you it took me about 3 readings to really get it.

Hence his contest.

> All of this makes Lemire’s algorithm a really good challenge for creating a more readable version. Ideally something that an average coder can read in one pass, understand, and agree that it’s correct. 

---

A couple things about all this intrigued me. First, I had just been [working on a project](https://sts10.github.io/2020/09/30/making-a-word-list.html) that dealt with choosing random words from a long list. Second, anything shorthanded with a last name, followed by the word "algorithm" seems pretty cool, especially if it's relatively new. 

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

On first blush, this made no sense to me. Like none. And even if it was in a language I can write I think I'd have still been completely lost. Thankfully, MacCárthaigh reassures:

> The second reason I chose Lemire’s algorithm is that it is impenetrable upon first reading. There are lucky few people who are so practiced and conversant in number manipulation that they can see inside of algorithms like Neo in the matrix, but I don’t mean them. To the average reader, myself included, it’s not clear what’s going on and why.

Check!

## Our starting point, our foothold

As MacCárthaigh notes in his post, he used the contestants' answers to write his own explanation in [a truly wonderful and very long code comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L188). This, rather than anything written by Lemire, was my starting point to understand this thing.

I read MacCárthaigh comment through at least three times, picking up the tiniest bit of new knowledge each time. I got frustrated when it was immediately clear to me what was happening and I got up and did other things. I cursed my curiosity, my lack of CS knowledge... cursed how I understood _just a little bit_ of it. With my attitude settled, I went slow and took each section of the comment in turn.

I also started throwing some Rust code in a playground, figuring I'd attempt an example of rolling a single 6-sided die as a way to learn.

## A note about Lemire and Rust

Pretty late into this little project of mine I learned that the main Rust library for generating random number, [Rand](https://github.com/rust-random/rand), apparently [took at least some ideas from Lemire back in 2018 for version 0.5.0](https://www.reddit.com/r/rust/comments/8l95zk/rand_050_released/). So it's not like I'm doing anything novel by implementing this algorithm in Rust. And in fact my ability to read Rust code isn't good enough to find where Lemire's work is used in the library. 

## Part 1: Unfair dice

In [the first section of the comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L196-L213), MacCárthaigh walks us through a basic example of what we'll ultimately doing. 

Basically this Lemire's algorithm is a fast way of using a randomly generated number to randomly pick another number from a range. 

In the comment's example, we've got a random number that's either 0, 1, 2, 3, 4, 5, 6, or 7. And what we want is "to use that to generate a number in the set 0, 1, 2."

The first thing we'll try is the modulus or remainder operator (`%`). MacCárthaigh observes: "A naive way to to this is to simply use x % 3. % is the modulus or remainder * operator and it returns the remainder left over from x/3." Why is it "naive"? We'll find out!

For my 6-sided die example, I would have used 0 through 7 for my large, initial random number (which I'm going to call a "seed" but MacCárthaigh refers to as `x`), but Rust doesn't easily support 3-bit numbers. 

The smallest bit number Rust easily supports is 8 bits (`u8`), which covers a range of 0 to 255. So I used that. In Rust I did this with the rand crate with `let seed_up_to_255 = rand::random::<u8>();`

IMPORTANT: Given the way bits and computers work, I think computers are very fast at generating random numbers if the range you want it is the length of 2^x. Since 256 = 2^8, generating this initial random number happens very quickly. The problem Lemire's code address is taking one of these "fast" random numbers and quickly adapting them to _any_ given range of numbers.

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

Basically, since 256 isn't a multiple of 6, we're going to roll 0, 1, 2, and 3 a little more often 4 and 5. And that's unfair. So how do we fix this?

## Rejection method 

Now we're on to what I'm called [the second section of MacCárthaigh's code comment, which is about rejection sampling](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L215-L234). 

Basically, one way to make this unfair die a fair die is to reject seeds of 252, 253, 254, and 255. That way, 0, 1, 2, 3, 4, and 5 all have an equal chance of being returned.

```rust
fn rejection_method() -> u8 {
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
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        if seed < ceiling {
            // Got a good seed, so we'll make it a dice roll and return it
            return seed % 6;
        } else {
            // Got a bad seed (too high)!
            // Return to the top of this loop to get a new seed
            continue;
        }
    }
}
```

This solves our problem!

But one bummer here is that in 4 our of 256 potential seeds, we have to do a do-over, which isn't ideal for efficiency. As MacCárthaigh notes: 

> This algorithm works correctly but is expensive. There's at least two % operations per call and maybe more, and those operations are among the slowest a CPU can be asked to perform.

So where's the magic sauce?!

## An important note in the code comment

This next section of the comment did NOT make sense to me at first and it's still a bit shakey (I might have gotten a pad and pen at this point), but it's definitely important from later. I'll paste it here so you stop and read it at least once or twice (remember, MacCárthaigh's `x` is what I've been calling "seed").

```text
With our code, we checked if x was between 0 and 5, because that's
easiest, but any contiguous window of 6 numbers would have done.
For example:

 x =         0  1  2  3  4  5  6  7
             +--+--+--+--+--+--+--+
                \_____/  \_____/
 x % 3 =        1  2  0  1  2  0

 x =         0  1  2  3  4  5  6  7
             +--+--+--+--+--+--+--+
                   \_____/  \_____/
 x % 3 =           2  0  1  2  0  1

There's a general principle at play here. Any contiguous range of
(n * s) numbers will contain exactly n values where x % s is 0, n values
where x % s is 1, and so on, up to n values where x % s is (s - 1).
This is important later, so really convince yourself of this.
```

In his example, x is what I'm calling the seed, s is 3 (for me it's 6 for sides of the die). So filling those in, we get: 

> Any contiguous range of (n * 3) numbers will contain exactly n values where [the seed] % 3 is 0, n values where [the seed] % 3 is 1, and so on, up to n values where 8 % 3 is (3 - 1).

I'm not really sure how to explain this further so I'm going to move on!

## Using a floor rather than a ceiling

One thing (a) I can get my head around and (b) will be helpful later is the idea that we could use a "floor" rather than a "ceiling" here. In other words, instead of rejecting seeds of 252, 253, 254, and 255; we could instead reject seeds of 0, 1, 2, and 3. 

To calculate that `3` floor, let's do `let floor = 255 % 6`. 

So all together it'd look like: 

```rust
fn rejection_method() -> u8 {
    // Another solution to this problem is to call a "do over" if the seed is too low, in this case 0, 1, 2 or 3
    let floor = 255 % 6;
    assert_eq!(floor, 3);
    // Now we can do ...
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        // compare this seed to our floor
        if seed > floor {
            // Got a good seed, so we'll make it a dice roll and return it
            return seed % 6;
        } else {
            // Got a bad seed (too LOW)!
            // Return to the top of this loop to get a new seed
            continue;
        }
    }
}
```

You can test this function for equal distribution [with this rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c2863ec2f3859fed16cb4a8e2855e17d).

(I'm not sure if this is the correct way to calculate `floor` for all possible dice...)

## Our first attempt at Lemire's algorithm

Alright, at this point MacCárthaigh deems us ready for the new stuff. In [the next section](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L259-L311) he introduces Lemire's algorithm, but he describes an **unfair** version of it. It's unfair in a way that's similar to our first dice implementation, but a little different. Let's explore!

```rust
fn lemire_unfair() {
    let seed = rand::random::<u8>(); // get a random number from 0..=255

    // Kind of blindly trusting the explanation of Lemire's algorithm,
    // we're going to calculate a variable named m like this:
    let s = 6
    let m: usize = seed as usize * s; // Note that the maximum value of m is 255 * 6 or 1,530

    // So m is a random number, with values that are multiples of 6:
    // 0, 6, 12, 18, 24, 30, etc.  up to 1,530

    // Note that we can easily get a dice roll (though not a fair one) from m by dividing it by 256
    let seed = rand::random::<u8>(); // get a random number from 0..=255
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
    let example_roll = m / 256;
    println!("Example roll using m and division: {}", example_roll);


    // But this method still unfair in a similar way that our initial roll function is
    // unfair.
    
    // For seeds from 0 to 42 (43 seed values), we get a dice roll of 0
    assert_eq!((42 * 6) / 256, 0);
    // For seeds from 43 to 85 (43 seed values), we get a dice roll of 1
    assert_eq!((43 * 6) / 256, 1);
    assert_eq!((85 * 6) / 256, 1);
    // For seeds from 86 to 127 (42 seed values), we get a dice roll of 2
    assert_eq!((86 * 6) / 256, 2);
    assert_eq!((127 * 6) / 256, 2);
    // For seeds from 128 to 170 (43), we get a dice roll of 3
    assert_eq!((128 * 6) / 256, 3);
    assert_eq!((170 * 6) / 256, 3);
    // For seeds from 171 to 213 (43), we get a dice roll of 4
    assert_eq!((171 * 6) / 256, 4);
    assert_eq!((213 * 6) / 256, 4);
    // For seeds from 214 to 255 (42), we get a dice roll of 5
    assert_eq!((214 * 6) / 256, 5);
    assert_eq!((255 * 6) / 256, 5);

    // It over-returns 0, 1, 3 and 4, and under-returns 2 and 5
}
```

### A trick to calculating m

Remember Lemire is all about speed. So there's a few times where he uses some computer science tricks to speed things up. For example, apparently thanks to the nature of u8 integers, dividing by 256 can also be done be using a "bit shift" to the right of 8.

In Rust, [a right shift is represented with >>](https://doc.rust-lang.org/book/appendix-02-operators.html), so right-shifting `m` by 8 `m >> 8`. We can pretty easily check this ourselves for all possible `u8` values by running the following:

```rust
for _n in 0..=255 {
    let seed = rand::random::<u8>(); // get a random number from 0..=255
    let m: usize = seed as usize * 6; // Note that the maximum value of m is 255 * 6 or 1,530
    assert_eq!(m >> 8, m / 256);
}
```

Going forward I'll try to use `m >> 8`. This little operation is conceptually important because it'll be the very last calculation for us, the one that acutally produces the 0 to 5 number. (In MacCárthaigh example, he's using 3-bit numbers, so he uses `m >> 3`.)

But no matter how you calculate `m`, our current method is still unfair. 

## A fair Lemire

Alright now we're getting dangerous. In [the next section](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L294-L338), MacCárthaigh introduces boats and the `l` variable. I definitely was using pen and graph paper at this point. So for better or worse I'm going to stop narrating along with the comment and more fully encourage you to read it.

For now, I'll paste some code I wrote as I made my way to a final implementation. 

For example, at around this point I broke my initial attempt at Lemire's into two functions: 

```rust
fn lemire_slow(seed: u8, s: usize) -> Option<usize> {
    let rand_range_length = 256; // range of seed
    let m: usize = seed as usize * s; // Note that the maximum value of m is 255 * 6 or 1,530
    let l = m % rand_range_length; // a new variable l!
    if l >= (rand_range_length % s) {
        // good seed, return the dice roll
        return Some(m >> 8);
    } else {
        // bad seed
        return None;
    }
}

fn roll_using_lemire_slow(dice_size: usize) -> usize {
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        match lemire_slow(seed, dice_size) {
            // if we get a result, that means we got a good seed and thus we got a roll
            Some(r) => return r,
            // got a bad seed and thus no roll. 
            // try loop again
            None => continue,
        };
    }
}
```

I like this because `lemire_slow` offers a Rust-y visual of when we reject a seed (return None)

## Medium Lemire 

Here's what was my next attempt, which uses the `floor` trick.

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

And finally here's my "fast" Lemire, where I decided to end my journey for now. 


```rust
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
    m >> 8
}
```

### Notes on Fast Lemire function and its two weird shortcuts

As the name of the function implies, this is where I did my best to implement any and all speed shortcuts described by Lemire and MacCárthaigh.

First, you'll notice I replaced the apparently slow `let l = m % 256;` with `let l: u8 = m as u8;`, which is apparently another one of those math/comp sci shortcut tricks. [MacCárthaigh explains it pretty well in the comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L336-L358), where the finished code is the even more cryptic `uint3_t l = (uint3_t) m;`.

Helpfully for my Rust implementation, this same trick is explained in a comment in [the Rust by Example page on casting](https://doc.rust-lang.org/rust-by-example/types/cast.html).

I also used `let floor: u8 = (u8::MAX - s + 1) % s;` where Lemire uses `uint64_t t = -s % s;`. MacCárthaigh [explains also this shortcut in his comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L393-L423). I got the Rust implementation with help from another Mastodon friend and a little luck.

And I proved it to myself this way:

```rust
fn main() {
    for s in 1..=7 as u8 { // doesn't work for 0 but think we're OK with that
        let slow_calc = 256 % s as u16;
        let fast_calc = (u8::MAX - s + 1) % s;

        assert_eq!(slow_calc as u8, fast_calc);
    }
}
```

I should probably run a benchmark to see if it matters. 

## Writing benchmark tests using the Criterion crate

Just for fun, I wanted to benchmark my beautifully named `roll_using_lemire_fast` function. Benchmarking against [Rust's Rand library](https://github.com/rust-random/rand) seemed a good choice, though I later learned that [that library already uses some of Lemire's work](https://www.reddit.com/r/rust/comments/8l95zk/rand_050_released/). But I pressed on.

To find out, I decided to learn a little about benchmarking Rust code, something I'd never _formally_ done before. After a few search quieries, I decided to use a crate called [Criterion](https://github.com/bheisler/criterion.rs).

Not going to lie, did a lot of copy and pasting from [its Getting Started page](https://bheisler.github.io/criterion.rs/book/getting_started.html). But here's what I ended up with in `./benches/my_benchmark.rs`:

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

And then I hastily wrote this in `lib.rs`:

```rust
pub fn roll_using_gen_range(dice_size: u8) -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(0, dice_size - 1)
}
```

Criterion showed my Lemire function beating `roll_using_gen_range` by a few nanoseconds. But since my assumption is that Rand's `gen_range` uses Lemire too, my guess is that the only reason my Lemire is faster is because Rand and `gen_range` are more versatile and complex.

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

## What about readability? 

MacCárthaigh original challenge was to make the algorithm more _readable_. 

In thinking about how I might make a more readable implementation, I returned to the structure I used for my first fair (but slow) implementation of Lemire above, which was broken into two functions.

I like two things about splitting it into two functions, an outer and an inner. First, it shows what we might call the "seed rejection" logic clearly: The outer function has the only loop, in which it passes a possible seed to an inner function. 

Having this inner function return a Rust `Option` seems like a nice choice for readability here: It can return `Some` result or `None`, which maps nicely to "a good

```rust
// Going to attempt to break up Lemire's into 4 or 5 functions for improved readability
#[inline]
pub fn roll_using_readable_lemire(s: u8) -> u16 {
    loop {
        let seed = rand::random::<u8>(); // get a random number from 0..=255
        match lemire_from_seed(seed, s) {
            // if we get a result, that means we got a good seed and thus we got a roll
            Some(roll_result) => return roll_result,
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
```

## Further work to do

First, I probably need to double check everything. I'd like to figure out how to write a test to confirm that `roll_using_lemire_fast` is fair. (Maybe a Chi-squared test?)

And obviously my function can only generate random numbers over a max range of 256. Lemire's original example code takes a 64-bit integer for its `s`, which makes the function much more practical and versatile. This requires an `m` of 128 bits, which I'll have to check if Rust can handle? Alternatively I could settle for a 32-bit `s`.

And lastly, returning to MacCárthaigh's original call for readability, I'd love to re-write my Lemire function to be more readable, even at the expensive of speed.

## Appendix: More sample code

Remember when I split my slow, basic Lemire's into two functions? I wrote a test to make sure it produces an even distribution of dice rolls:

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


