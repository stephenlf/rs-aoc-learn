# Day 8: Treetop Tree House
> **KEY CONCEPTS**
> - 2D Vectors with `Vec<Vec<_>>`
> - Multithreading
> - Benchmarking with [Criterion](https://crates.io/crates/criterion)

For today's challenge, we will be analyzing a 2D grid of tree heights to determine what trees are visible from where. In our code, we will model this grid as a 2D vector, or `Vec<Vec<_>>`.

Besides the `Vec<Vec<_>>` construct, there isn't much Rust that we _have_ to use that we haven't already seen. So instead, we'll spice up the challenge by implementing **two** solutions. One will be single-threaded, and one will be **multi-threaded** using Rust's [fearless concurrency model](https://doc.rust-lang.org/book/ch16-00-concurrency.html).

It isn't enough to just solve this puzzle twice, though. We also need to know which one is better. To do this, we'll be using the [Criterion](https://crates.io/crates/criterion) crate, a powerful, statistical microbenchmarking tool created by Rust community members.

Let's begin!

```bash
# aoc
$ cargo new day_8
$ cd day_8

# add shared library
$ cargo add aoc --path ".."
```