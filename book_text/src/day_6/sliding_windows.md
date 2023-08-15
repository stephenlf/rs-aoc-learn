# Sliding Windows
The Rust standard library provides us with the `std::slice::windows` function, which will let us iterate over overlapping _windows_ of a [slice](https://doc.rust-lang.org/core/primitive.slice.html#). For example, given the following character slice `slice: &[char]` and a window size of `2`, calling `slice.windows()` would look something like this (example pulled from the docs):

```rust 
let slice = ['r', 'u', 's', 't'];
let mut iter = slice.windows(2);
assert_eq!(iter.next().unwrap(), &['r', 'u']);
assert_eq!(iter.next().unwrap(), &['u', 's']);
assert_eq!(iter.next().unwrap(), &['s', 't']);
assert!(iter.next().is_none());
```

> Compared to yesterday's problem, today's problem is rather easy in my opinion. I invite you to try solving the puzzle using what you can find about the `windows` function in the standard library. Once you have done that, check out my explanation below.

With this function, we can iterate over subslices of our input to find a string of four unique characters. Let's see what that iterator looks like.

```rust
// aoc/day_6/main.rs
fn find_signal_start(signal: String) -> usize {
    let signal: Vec<char> = signal.chars().collect();
    
    for window /*: &[char; 4]*/ in signal.as_slice().windows(4) {
        todo!()
    }
}
```

Since `windows` is a `slice` method and not a `&str` method, we first have to turn our `String` into a sliceable object, namely, `Vec<char>`. We can turn `Vec<char>` into `&[char]` by calling the `as_slice` method. The result is an iterator over `&[char]`. 

Within the body of our loop, we check if the current item is composed of all unique characters. If it is, we return the iterator's index plus 4 (`i + 4` instead of `i` because our iterator at `i = 0` has already looked at `4` characters). We'll need to call `enumerate` on our `windows` iterator to get the index at each step.

```rust
// aoc/day_6/main.rs
fn find_signal_start(signal: String) -> usize {
    let signal: Vec<char> = signal.chars().collect();
    
    for (i, window) in signal.as_slice()
        .windows(4)
        .enumerate() 
    {
        if is_unique(window) {
            return i + 4;
        }
    }
    panic!("Could not find a valid signal start sequence");
}


```

Finally, we need to create an implementation for our `is_unique` function. This function checks if every character in the input slice is unique, and returns `true` or `false` accordingly. Here's what I came up with. This code checks if a `std::collections::HashSet` of the slice is 4 characters long, since `HashSet`'s can only have unique characters.

```rust
// aoc/day_6/main.rs
use std::collections::HashSet;
// ..
fn is_unique(window: &[char]) -> bool {
    assert_eq!(window.len(), 4);

    // Throw each character into a set. 
    let mut set: HashSet<char> = HashSet::with_capacity(4);
    for c in window {
        set.insert(*c);
    }

    // Sets only keep unique entries, so a set length of four means that all 
    // entries were unique. Depends on the assertion that window.len() == 4.
    if set.len() == 4 {
        true
    } else {
        false
    }
}
```
With this our business logic prepared and tested, we can now solve the puzzle. I will use the `std::fs::read_to_string` to get our input loaded. It's a bit simpler than what we've been using for previous problems.

```rust
// aoc/day_6/main.rs
// ..
fn main() {
    let input_path = std::path::PathBuf::from("../inputs/day_6.txt");
    let input = std::fs::read_to_string(input_path).unwrap();
    println!("Day 1: {}", find_signal_start(input));
}
// .. 
```

Great work! This was a pretty easy problem, which is nice since tomorrow's will be pretty hard. Fun!