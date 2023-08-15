# Day 7: No Space Left On Device 
> **KEY CONCEPTS**
> - The _borrow checker_ ❗
> - Mutable shared references with the `RefCell<Rc>` pattern
> - The `Display` trait (again)

For today's challenge, we are asked to simulate a virtual filesystem with file names, file sizes, and nested folders. We will use the challenge input to populate our filesystem, then we will use the filesystem to answer questions about folder sizes.

This problem is the hardest challenge we've encountered so far, and the hardest challenge we will encounter for a while. The key problem we have to overcome is solving the _reference issue_, namely, how can we hold on to multiple, mutable references to the same object? In solving this, we will see how Rust's **borrow checker** makes this pattern difficult, what tools Rust gives us to make this pattern easy, and ultimately why Rust's approach makes for a better development experience overall.

Warning, this tutorial will have a lot of code. I know that the only thing as strong as a programmer's love for reading code is their hatred for reading code. But this might be our important lesson so far, and you will benefit greatly from understanding all of the code snippets. You got this, _champion_.

Let's begin.

```bash
# aoc
$ cargo new day_7
$ cd day_7

# add shared library
$ cargo add aoc --path ".."
```