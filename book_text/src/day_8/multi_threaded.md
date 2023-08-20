# A Multi-Threaded Solution

> In many languages, multithreading is hard. [Race conditions](https://en.wikipedia.org/wiki/Race_condition) can make code varingly work and not work depending on the weather. And poorly synchronized memory allocators can cause segmentation faults. If programmers aren't careful, they'll find themselves with a serious headache trying to solve these problems in production.
> 
> Fortunately, Rust's borrow checker saves that day! Rust's compile-time mutable reference checks makes sure that no data is read while it is being modified, helping prevent race conditions. And the borrow checker's ownership model makes sure that all memory being accessed it valid. 


Let's extend our `Forest` API to include the method `calc_multi`, which is a multi-threaded implementation of our `calc_visibility` function. We don't know exactly what the business logic will look like for this function, but we know that its signature will match `calc_visibility`. 

```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    // ..
    pub fn calc_multi(&mut self) {
        todo!();
    }
}
```
Now, how can we use multithreading to solve this puzzle?

> **WARNING:** The code you're about to see is _not_ going to be the best way to solve this puzzle. But that's the beauty of benchmarking, right? We get to try stupid stuff and see if it works.

There are two broad approaches to multithreaded programming: [message passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html) and [shared state](https://doc.rust-lang.org/book/ch16-03-shared-state.html). Both are described in detail in _The Rust Programming Language_. We we be using a mix of both for today's solution.

Here's an overview of what our solution will look like.
1) Spin up 4 threads
2) With each thread, scan the `Forest` from each of the four directions, calculating visibility for each `Tree`
3) Pass the visibility of each `Tree` back to the main thread through a channel
4) Update the visibility of each `Tree` with the main thread

You may notice a problem with our algorithm right off the bat. The borrow checker won't like that our threads are reading from data that our main thread is writing to. There are convoluted ways we can try to get around this, e.g. by wrapping each `Tree` in an `Arc<Mutex<_>>` smart pointer. But to keep things simple, let's just copy all of the `Tree` heights into a new, immutable object.

```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    // ..
    pub fn calc_multi(&mut self) {
        let heights: Vec<Vec<i32>> = self.0.iter().map(|row| {
            row.iter().map(|tree| tree.height).collect()
        }).collect();
    }
} 
// ..
```
We can pass our `heights` object to different threads by wrapping it in the `Arc` smart pointer. `Arc` will take care of any lifetime issues by making sure that `heights` isn't dropped before each thread is using it.

```rust
// aoc/day_8/src/forest.rs
use std::sync::Arc;
// ..
impl Forest {
    // ..
    pub fn calc_multi(&mut self) {
        let heights: Vec<Vec<i32>> = self.0.iter().map(|row| {
            row.iter().map(|tree| tree.height).collect()
        }).collect();

        let heights = Arc::new(heights);
    }
} 
// ..
```
Now we can spawn our threads and pass in clones of our `heights` pointer for each thread to read from.

Before we spawn these threads, though, let's also prepare a _channel_ through which our spawned threads may communicate with the main thread. We can do that with Rust's `mpsc` [module](https://doc.rust-lang.org/std/sync/mpsc/).

```rust
// aoc/day_8/src/forest.rs
use std::sync::Arc;
use std::sync::mpsc;
// ..
impl Forest {
    // ..
    pub fn calc_multi(&mut self) {
        let heights: Vec<Vec<i32>> = self.0.iter().map(|row| {
            row.iter().map(|tree| tree.height).collect()
        }).collect();

        let heights = Arc::new(heights);

        // Threads will send messages of the form (row, column) 
        // for each visible tree
        let (tx, rx) = mpsc::channel::<(usize, usize)>();
    }
} 
// ..
```
We can now start spawning threads to calculate visibility from each of the four directions. We will have to pass in a copy of our `heights` pointer and `tx` transmitter to each thread. This will let our side threads read the data and transmit results across threads, respectively. 

We'll define a single function, `read_from`, which takes in the `heights` pointer, `tx` transmitter, and a direction, and starts calculating the visibility of each tree from the given direction. If a tree is visible, it passes its coordinates to the transmitter to be processed by the main thread.

Here's what the thread spawning looks like \[for brevity, code is limited to only `Bottom` and `Top` implementations\].
```rust
// aoc/day_8/src/forest.rs
use std::{
    sync::{Arc, mpsc},
    thread
};
// ..
impl Forest {
    // ..
    pub fn calc_multi(&mut self) {
        // ..
        // Threads will send messages of the form (row, column) 
        // for each visible tree
        let (tx, rx) = mpsc::channel::<(usize, usize)>();

        let heights_clone = heights.clone();
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || 
            Self::scan_from(Direction::Bottom, heights_clone, tx_clone)
        );
        
        let heights_clone = heights.clone();
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || 
            Self::scan_from(Direction::Top, heights_clone, tx_clone)
        );
        // Abbreviated..
    }
} 

enum Direction {
    Left,
    Right,
    Top, 
    Bottom,
}
// ..
```
And here's what a partial implementation of the `scan_from` function might look like \[again shortened for brevity].
```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    fn scan_from(direction: Direction, tree_grid: Arc<Vec<Vec<i32>>>, tx: mpsc::Sender<(usize, usize)>) {
        let (rows, columns) = (tree_grid.len(), tree_grid[0].len());
        let mut max_height: i32;

        match direction {
            Direction::Left => {
                for i in 0..rows {
                    max_height = -1;
                    for j in 0..columns {
                        let tree_height = tree_grid[i][j];
                        if tree_height > max_height {
                            max_height = tree_height;
                            tx.send((i, j)).unwrap();
                        }
                    }
                }
            },
            Direction::Top => {
                for j in 0..columns {
                    max_height = -1;
                    for i in 0..rows {
                        let tree_height = tree_grid[i][j];
                        if tree_height > max_height {
                            max_height = tree_height;
                            tx.send((i, j)).unwrap();
                        }
                    }
                }
            },
        // Abbreviated ..
        }
    }
}
```
Great! With our side threads working away, we can now use our main thread to start updating our `Forest`. We can iterate over messages in our `rx` receiver in a loop.
```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    // ..
    pub fn calc_multi(&mut self) {
        // ..

        // Required for loop to finish
        drop(tx);

        for (i, j) in rx {
            self.0[i][j].visibility = true;
        }
    }
    // ..
}
// ..
```
And that's it! Our `calc_multi` function is now ready as a drop-in replacement for our `calc_visibility` function. Let's throw it in our main function and see if it's working.
```rust
// aoc/day_8/src/main.rs
mod forest;

fn main() {
    let mut forest = forest::Forest::new("../inputs/day_8.txt");
    forest.calc_visibility();
    println!("Part 1: {}", forest.sum_visibility());

    let mut forest = forest::Forest::new("../inputs/day_8.txt");
    forest.calc_multi();
    println!("Part 1 (multithreaded): {}", forest.sum_visibility());
}
```
Checking the output, we get the following:
```bash
# aoc/day_8
$ cargo run
Part 1: 1543
Part 1 (multithreaded): 1543
```
Amazing! We took what was a simple 2D vector function and made it a multithreaded, multiparadigm algorithm. Very nice. But is our new algorithm better? Let's run a benchmark to find out.