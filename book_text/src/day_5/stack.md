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
// aoc/day_5/main.rs
// ..
fn parse_command(line: &String) -> (u32, usize, usize) {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let num_crates = tokens[1].parse::<u32>()
        .expect("Could not parse num_crates");
    let origin = tokens[3].parse::<usize>().unwrap();
    let dest = tokens[5].parse::<usize>().unwrap();
    
    (num_crates, origin, dest)
}

fn main() {
    // ..
    for line in lines {
        let (num_crates , origin , dest) = parse_command(&line.unwrap());
        port.arrange(num_crates, origin, dest);
    }
}
```
Checking to see if it works returns an error. This is because the first `line` that we pull in our iterator is the blank spacer line between the port diagram and the command list. A simple `lines.next()` call will consume that for us.

```rust
// aoc/day_5/main.rs
// ..
fn main() {
    // ..

    let _ =lines.next();

    for line in lines {
        // ..
    }
}
```
And finally, we fill out our `Port::print` function, which prints out the top crate of each `Dock`.
```rust
// aoc/day_5/port.rs
// ..
impl Port {
    // ..
    pub fn print(&self) {
        print!("Part 1: ");
        for dock in self.0.iter() {
            print!("{}", dock.0.last().unwrap());
        }
    }
    // ..
}
// ..
```
Now we can solve the puzzle.
```rust
// aoc/day_5/main.rs
// ..
fn main() {
    // ..
    port.print();
}
```
Good work! That was a bit more challenging than the previous puzzles, but we got it.

## Bonus content: The `Display` trait
Our call to `Port::print` works fine, but it seems a little... _weird_. We already have great tooling to print out objects. Can't we use one of those?

Yes... kinda. In the standard library we have the `std::fmt::Display` trait, which defines how objects are converted to strings in formatting macros. In other words, by implementing `std::fmt::Display` for `Port`, we can define the behavior of the following code:
```rust
let port = port::Port::new(/*..*/);
println!("{}", my_port);    // Notice no `Debug` formatter specified.
        // This pattern will also now work in `print!()`, `format!()`, etc...
```
Let's look at the [implementation](https://doc.rust-lang.org/std/fmt/trait.Display.html) for the `Debug` trait.
```rust
pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", todo!())
    }
}
```
The `write!()` macro is built similarly to the `format!()` macro, with an additional parameter `f` passed in by the `fmt` function. All we have to do is replace the `todo!()` call with the string we wish to return when we print our `Port`. This can be anything you want. For example, you can simply copy and past the `Port::print` function implementation here. Or you can recreate the port diagram provided by the input. (For an implementation that recreates the input diagram, check out the source code. I'm pretty proud of it!)
