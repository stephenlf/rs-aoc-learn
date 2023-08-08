# Stacks: Push and Pop
With our `Port` created, we can move on to implementing our `Port::arrange` function. This function should `pop` off the top crate from the `Dock` at index `origin`, `push` the result onto the `Dock` at index `dest`, and repeat `num_crates` times. Seems simple enough! 
```rust
// aoc/day_5/port.rs
// ..
impl Port {
// ..
    /// Moves `num_crates` number of crates from the top of stack `origin`
    /// to the top of stack `destination`. Crates are moved one at a time.
    /// Origin and dest indices are offset by one to match input.
    pub fn arrange(&mut self, num_crates: u32, origin: usize, dest: usize) {
        for _ in 0..num_crates {
            let c = self.0[origin - 1].0.pop().unwrap();
            self.0[dest - 1].0.push(c);
        }
    }
// ..
}
```
Let's also add a loop in our main function to parse out each line of input and run it through this `arrange` function.
```rust

```