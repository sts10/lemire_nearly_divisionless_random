# An exploration of Lemire's nearly divisionless random using Rust

TL;DR As a learning experience, I tried write my own implementation of [Daniel Lemire's nearly divisionless random](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/) in Rust. I don't _quite_ understand it as fully as I'd like to yet, but this is where I'm at.

**Do NOT use this code for production!** It's a learning tool, written by an amateur programmer. And for another thing it's limited to 8-bit integers.

## Organization of files

`notes.markdown` is a long Markdown document containing my process of trying to understand Lemire's nearly divisionless random, which largely follows [this very helpful code comment](https://github.com/colmmacc/s2n/blob/7ad9240c8b9ade0cc3a403a732ba9f1289934abd/utils/s2n_random.c#L188) by Colm MacCárthaigh. A good place to start!

### Rust files

To run these, you'll need to have [Rust and `cargo` installed](https://www.rust-lang.org/tools/install).

- `src/readable.rs` is a Rust module that contains that 4 functions that, together, make up what I think is **my best implementation** of Lemire's nearly divisionless random, which prioritizes _readability_. (This file also contains 4 unit tests, which you can run with `cargo test`.)
- `src/lib.rs` has an implementation closer to Lemire's C version, as well as a more "traditional" solution to the problem.
- `benches/` directory contains the benchmarks, written using the [Criterion crate](https://docs.rs/criterion/0.3.3/criterion/). You can run them with `cargo bench`.
- `src/main.rs` is a simple example of how you'd use my "readable" implementation. You can run it with `cargo run`.



