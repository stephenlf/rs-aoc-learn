# My First Workspace

> _pwd: **aoc**_
>
> Topics covered: [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html#virtual_worksp)

Advent of Code presents 25 challenge problems for us to solve, each with a part A and part B. That's up to 50 programs that we'll be writing! We could write all of these programs in a single package, with one common _lib_ crate and 50 _bin_ crates. However, that starts to get slow to compile and difficult to make sense of. Instead, let's use Cargo's _workspace_ feature.

Workspaces allow us to combine multiple packages within a single cargo project. To start, create a `cargo.toml` file and add the `[workspace]` section. 

```toml
# aoc/cargo.toml

[workspace]
members = [./]
```

The `members` lines tells cargo that all packages initialized within our workspace directory belong to our workspace. 

> **Challenge: Read the documentation on _cargo.toml_ and add appropriate workspace metadata to your manifest.**