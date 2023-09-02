# Day 11: Monkey in the Middle
> **Key Concepts**
> - Closure types `dyn Fn`, `dyn FnMut`, and `dyn FnOnce`
> - Double-ended queue with `VecDeque`
> - Custom ordering with the `Ordering` trait

We've spent the last few challenges diverging from Rust's standard library in order to keep things interesting. Fortunately, today's challenge doesn't need any spice; it's interesting all on its own.

In this challenge, we are given a list of monkeys, each with some number of starting `items`, an `operation` to perform on each item, and a `test` to run the operation's output through. To encode the `operation` and `test`, we'll be taking advantage of Rust's [__closures__](https://doc.rust-lang.org/rust-by-example/fn/closures.html). We'll also explore Rust's doubly-linked list `std::collections::VecDeque` and an implementation of Rust's `Ordering` trait.

This is exciting! Let's start.

```bash
# aoc
$ cargo new day_11
$ cd day_11

# add shared library
$ cargo add aoc --path ".."
```
