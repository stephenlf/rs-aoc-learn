# Data Modeling with Structs: Pt. II
Parsing our input will be a bit tricky, but with clear data modelling and well-tested functions, we shouldn't hit any issue that we can't solve. 

Our problem's setup can be modelled as a single `Port` containing many stack's of crates. We can imagine each stack of crates came off of a ship moored at a different dock, so we'll call these stacks `Dock`'s. Each crate in our `Dock`'s is identified with a single `char`. Let's set up these relationships in our main file.
```rust
// aoc/day_5/src/main.rs
struct Port(Vec<Dock>);

struct Dock(Vec<char>);
```
I anticipate there will be quite a bit of code associated with these structs later, so let's wrap them in a `mod`ule. This will give them their own namespace and generally keep our code cleaner.
```rust
// aoc/day_5/src/main.rs
mod port {
    struct Port(Vec<Dock>);

    struct Dock(Vec<char>);
}
```
In fact, there's no reason to keep the contents of the `port` module in our _main_ file. Let's drop it in a new file at `aoc/day_5/src/port.rs`. Naming is important here, as cargo will look in either `src/port.rs` or `src/port/mod.rs` for the definition of our `port` module, and will throw an error if it can't find it. We'll also need to keep a declaration of our new `port` module in our _main_ function.
```rust
// aoc/day_5/src/port.rs    <-- DIFFERENT
struct Port(Vec<Dock>);

struct Dock(Vec<char>);
```
```rust
// aoc/day_5/src/main.rs
mod port;
```
To continue our data model, let's define two methods that we'll use to solve our puzzle. One method, `arrange`, will take in the arguments of a single line of our input and rearrange the contents of each `Dock` accordingly. The other method will print out the top crate of each stack, giving us our final answer. We should also define some sort of `builder` function to create a port from our input.
```rust
// aoc/day_5/src/port.rs
struct Port(Vec<Dock>);

impl Port {
    /// Creates a new port from ...some input???
    fn new() -> Self {
        todo!()
    }

    /// Moves `num_crates` number of crates from the top of stack `origin`
    /// to the top of stack `destination`. Crates are moved one at a time.
    fn arrange(&mut self, num_crates: u32, origin: usize, dest: usize) {
        todo!()
    }

    /// Prints top crate of each Dock to stout
    fn print(&self) {
        println!("{}", todo!());
    }
}

struct Dock(Vec<char>);
```
Most items in modules are _private_ by default. This is a safety measure; it would do no good if our programmers could modify the contents of our `struct`'s arbitrarily, right? (_Ahem... [Python](https://python-guide-chinese.readthedocs.io/zh_CN/latest/writing/style.html#we-are-all-consenting-adults)_). Let's mark our business functions, methods, and `struct`'s public. Any other code we write from here on out will be internals only, and can stay private.

```rust
// aoc/day_5/src/port.rs
pub struct Port(Vec<Dock>);

// ..
    pub fn new() -> Self {
    // ..
    pub fn arrange(&mut self, num_crates: u32, origin: usize, dest: usize) {
    // ..
    pub fn print(&self) {
// ..
struct Dock(Vec<char>);
```
> _Should we make the `Dock` public?_
>
> I argue that `Dock`'s should stay private, since there are no public functions that take `Dock`'s as inputs, and no public methods under our `Dock` struct. The `Dock` will only exist in our code's internals.

## A new port
I must admit, parsing our input into a `Port` is a real head scratcher. Unlike previous problems, the input lines describing our `Port`'s initial condition depend on each other. Crates on line 3, for example, will need to be pushed onto our `Dock`'s before the crates on line 2 in order to preserve proper first-in, last-out ordering. Alternatively, we can create our `Dock`'s backwards and `reverse` them after initialization.

We also don't necessarily know how many `Dock`'s we need to prepare ahead of time. Sure, we can take a peek at our input and hardcode the dock number, but then we would need to use different code for our main input and the code examples provided in the problem body. Instead, we will use math to calculate the number of docks from the _character length_ of the first line of our input. 

### Calculating the number of docks
Our input visualizes the port's starting state in the following way:
```    
[C] [W]    
[A] [M] [D]
 1   2   3 
```
Docks are aligned as columns, with the contents of the docks stacked from top to bottom. Fortunately for us, each row of this dock visualization have exactly the same number of characters, and that character count depends on the number of docks (3 characters per dock, plus a 1 character spacer between docks). We can derive the following equation to calculate the number of docks:
```
c = 4d - 1
d = (c + 1) / 4
where d: number of docks
      c: number of characters in input line
```
For example, the input provided above has 11 characters per line, or `(11 + 1) / 4 = 3` docks. Looks good!

Let's turn that into a Rust function.
```rust
// aoc/day_5/src/ports.rs 
// ..
impl Port {
    // ..
    fn num_docks(line: &String) -> usize {
        (line.len() + 1) / 4
    }
}
// ..
```
We can then use this function to instantiate the appropriate number of docks from an input string.
```rust
// aoc/day_5/src/ports.rs 
// ..
impl Port {
    // ..
    fn create_docks(&mut self, line: &String) {
        for _ in 0..Self::num_docks(line) {
            self.0.push(Default::default());
        }
    }
    fn num_docks(line: &String) -> usize {
        (line.len() + 1) / 4
    }
}

#[derive(Default)]
struct Dock(Vec<char>);
```

### Grabbing crate IDs from input lines
Let's also define a function that can add the appropriate crate to each dock given an input line. Here, we'll iterate over every 4th `char` of the input string, starting at index 1. This will give us each alphabetic crate identifier, or a blank space if no crate exists at that index. If that character is alphabetic, we add it to the appropriate `Dock`.
```rust
impl Port {
    // ..
    fn populate_dock_from_line(&mut self, line: &String) {
        let mut chars = line.chars();

        // Get first character.
        let mut c = chars.nth(1).unwrap();
        self.0[0].0.push(c);

        let num_docks = self.0.len();

        for i in 1..num_docks {
            c = chars.nth(3).unwrap();
            self.0[i].0.push(c)
        }
    }
}
```
### Bringing it together
With this function defined, we are about ready to define a `new` function for our `Port`. I invite you to give it a go yourself. Here is the general algorithm:
1) The function takes a mutable reference to an iterator over the lines of our input as its only parameter.
1) From the first line, instantiate the appropriate number of `Dock`'s.
1) Loop over each line, populating the dock as you go.
1) When the iterator reaches the line containing all numbers and whitespace, end the loop.
Make sure to take in a mutable reference to your 
---
For my implementation, I will be turning our iterator into a `Peekable` [iterator](https://doc.rust-lang.org/std/iter/struct.Peekable.html). This will give us an additional function to use, `peek`, that will return a reference to the next value without consuming an iteration. This will be useful while instantiating our `Dock`'s.
```rust
// aoc/day_5/port.rs
// ..
impl Port {
    pub fn new(lines: &mut Peekable<LinesIter>) -> Self {
        let mut port = Self(Vec::new());
        let line: &String = lines.peek()    // Option<&Result<String>>
            .unwrap()       // &Result<String>
            .as_ref()       // Result<&String>
            .unwrap();      // &String

        port.create_docks(line);        // Empty docks instantiated

        loop {
            let line = lines.next() // Option<Result<String>>
                .unwrap().unwrap();

            // Check if first character is whitespace, indicating 
            // end of port diagram
            if line.chars().nth(0).unwrap().is_whitespace() {
                break;
            }

            port.populate_dock_from_line(&line);
        }

        // Docks were instantiated backwards, so we have to reverse them
        for dock in &mut port.0 {
            dock.0.reverse();
        }

        port
    }
}
// ..
```
With appropriate tests, we can see that our code is working! All we need to do now is create a `Port` in our main file.
```rust
// aoc/day_5/main.rs
use aoc;
use std::iter::Peekable;

mod port;

fn main() {
    let mut lines: Peekable<aoc::LinesIter> = 
        aoc::read_as_lines("../inputs/day_5.txt").unwrap().peekable();

    let mut port = port::Port::new(&mut lines);

}
```
Great work! Now we're ready to start moving our crates around.