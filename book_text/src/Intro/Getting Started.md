# Using This Book

This book assumes that you have read _[The Rust Programming Language](https://doc.rust-lang.org/book/)_ and have some mastery over basic Rust syntax. It also assumes that you have some experience building and running Rust programs (binaries and libraries), though I will be including all of the commands I use throughout the book. 

## Solution design philosophy
Whenever possible, I will focus these tutorials on the following principles:
1) Focus on learning Rust over algorithms.
2) Focus on instructive solutions over good solutions.
3) Parse our inputs programmatically. No hard coding solutions.

With few exceptions, I will only be providing tutorials for part 1 of each challenge puzzle. This is because solving part 2 usually requires tweaking the algorithms developed in part 1, which doesn't require learning any new Rust.

## Getting started

For licensing reasons, I won't be copying the Advent of Code 2022 (AoC) problems wholesale into this book, so you will need an AoC account to follow along. Make an account [on the website](https://adventofcode.com/).

You'll also need __rustup__ and __cargo__, the Rust compiler and package manager, respectively. I also **_strongly_** recommend installing the Rust Analyzer language server. The Rust Analyzer has been an important learning tool for me by giving me function signatures, available methods, type annotations, and syntax checks without ever leaving my code editor.

[Create Advent of Code account](https://adventofcode.com/)

[Install rustup/cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

[Rust Analyzer](https://rust-analyzer.github.io/) [[VSCode extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)]

We'll be using a Cargo _[workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html)_ to manage our projects. Using a _workspace_ means that we will have a separate crate for each AoC challenge, which will make compilation a little faster. I'm calling my workspace `aoc`. 

Let's begin.

```bash
$ mkdir aoc
$ cd aoc
```
