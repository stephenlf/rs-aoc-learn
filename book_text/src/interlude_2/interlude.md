# Interlude: A Shift from Fundamentals

If you've come this far, **amazing work**! We have learned quite a bit of Rust together.

Let's review some of the bigger concepts we've learned so far:
- Cargo project management
- Modeling data with `struct` and `enum`
- File IO, string parsing, and pattern matching
- Testing with `#[cfg(test)]` blocks
- `Vec`'s, iterators, and `HashSet`'s
- `Display` and `TryFrom` traits
- The borrow checker and `Rc<RefCell<_>>` pattern
What an impressive list! 

As we move on with Advent of Code's challenges, we will start to repeat a lot of the same tools and patterns we used to solve previous problems. For example, _ten_ of the following challenges will have some sort of 2D or 3D vector incorporated into the prompt (days 8, 9, 10, 12, 14, 15, 17, 18, 22, 23, and 24).

Rather than repeat the same tutorials over and over, I will instead shift the focus of this book away from Rust's fundamental structures and patterns in favor of learning Rust's powerful ecosystem of community-developed [crates](https://crates.io). 

That's not to say we won't still be learning Rust. We still have some fundamentals to learn, like the `Error` trait, parallel processing, state machines, comparison and ordering traits, `VecDeque`, and closures and closure types. But those will all be icing on the cake of such topics as:
* Benchmarking with [Criterion](https://crates.io/crates/criterion)
* Game design with [Bevy](https://crates.io/crates/bevy) and the entity-component system
* Async programming with [Tokio](https://crates.io/crates/tokio)
* Ergonomic Error handling with [anyhow](https://crates.io/crates/anyhow) and [thiserror](https://crates.io/crates/thiserror)
* Serialization and deserialization with [serde](https://crates/io/crates/serde) [?]
* Designing macros with [syn](https://crates.io/crates/syn) [?]

Some of these crates and toolchains won't be relevant to your work. Not everybody will be making a game with **Bevy**, or designing their own deserializers with **serde**. If that's the case, it's not especially important that you follow along in your own editors. However, I believe that exposure to these topics will make for a more rounded programmer. You will have a deeper understanding of the patterns that Rustaceans much more experienced than I use to tackle their real-life, production-level problems.

So if you're ready to learn, let's move on!
