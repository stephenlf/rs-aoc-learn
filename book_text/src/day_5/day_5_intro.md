# Day 5: Supply Stacks
> **Key Concepts:**
> [File](https://doc.rust-lang.org/rust-by-example/mod/split.html) [modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) 
> `Vec::push` and `Vec::pop`
> `std::iter::Peekable`
Today's challenge is a bit of a doozy--not to understand, but rather to model and parse. We are given a set of stacked crates, or characters. Crates will be transferred from stack to stack according to the commands given in our input, and we must determine which crates are left at the top of each stack when all is said and done. 

In fact, a _stack_ of crates is an apt model for how we'll represent these crates in our program. In computer science, a _stack_ is a set of items stored in a first-in, last-out basis. Typically, memory is stored and retrieved by _pushing_ and _popping_ items from the stack. We will be using Rust's `Vec` as our stack.

Modelling and parsing our input will take quiet a bit of code, so we are going to create a module in a new file that holds our `struct`'s and methods.

Let's begin.
```bash
# aoc
$ cargo new day_5
$ cd day_5

# add shared library
$ cargo add aoc --path ".."
```