# A Multi-Threaded Solution

Before we move on to implementing our multithreaded solution, let's do a bit of cleanup. We want to be able to run microbenchmarks on our solutions so that we know which one works best. To that end, I think it will be best if our business logic is confined to a single API call. 

Let's go back to our single-threaded solution and create a new `Forest` method, `solve_single`. This method will run `calc_visibility` and `sum_visibility` for us and return just the answer to our puzzle. 

```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    // ..
    pub fn solve_single(&mut self) -> u32 {
        self.calc_visibility();
        self.sum_visibility()
    }
    // ..
}
// ..
```
With that in place, we can now drop `solve_single`'s supporting functions like `calc_visibility` and `sum_visibility` behind a module named `single_thread`. Let's cut both of those methods out of _forest.rs_ and into _forest/single\_thread.rs_.

```rust
// aoc/day_8/src/forest.rs
mod single_thread;
// ..
```

```rust
// aoc/day_8/src/forest/single_thread.rs    <-- DIFFERENT
impl super::Forest {
    pub(super) fn calc_visibility(&mut self) {
        // ..
    }

    pub(super) fn sum_visibility(&self) -> u32 {
        // ..
    }
}
```
Note that we mark the `Forest` methods in the `single_thread` module `pub(super)`. This makes them available for use by their parent module (`forest`), but not available to our main API. We specifically need this so that `Forest::solve_single` can call them.

With that in place, we can change our `main` function a bit to account for the new API.
```rust
// aoc/day_8/src/main.rs
fn main() {
    let mut forest = forest::Forest::new("../inputs/day_8.txt");
    println!("Part 1: {}", forest.solve_single());
}
```
Great! Let's build our multi-threaded solution now.

## A `multi_thread` module
Let's extend our `Forest` API to include the method `solve_multi`, which implements a multi-threaded solution to our challenge. We don't know exactly what the business logic will look like for this function, but we know that its signature will match `solve_single`. Let's also make a `multi_thread` submodule to hold all of `solve_multi`'s supporting code.

```rust
// aoc/day_8/src/forest.rs
// ..
mod multi_thread;
// ..
impl Forest {
    // ..
    pub fn solve_multi(&mut self) -> u32 {
        todo!();
    }
}
```
Now, how can we use multithreading to solve this puzzle?

> **WARNING:** The code you're about to see is _not_ going to be the best way to solve this puzzle. But that's the beauty of benchmarking, right? We get to try stupid stuff and see if it works.

There are two broad approaches to multithreaded programming: [message passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html) and [shared state](https://doc.rust-lang.org/book/ch16-03-shared-state.html). Both are described in detail in _The Rust Programming Language_. We we be using _message passing_ for today's solution.

Under the message passing paradigm....

> In many languages, multithreading is hard. [Race conditions](https://en.wikipedia.org/wiki/Race_condition) can make code varingly work and not work depending on the weather. And poorly synchronized memory allocators can cause segmentation faults. If programmers aren't careful, they'll find themselves with a serious headache trying to solve these problems in production.
> 
> Fortunately, Rust's borrow checker saves that day! Rust's compile-time mutable reference checks makes sure that no data is read while it is being modified, helping prevent race conditions. And the borrow checker's ownership model makes sure that all memory being accessed it valid. 

