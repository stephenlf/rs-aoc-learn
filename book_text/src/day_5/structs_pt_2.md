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
I must admit, parsing our input into a `Port` is a real head scratcher. Unlike previous problems, the input lines describing our `Port`'s initial condition depend on each other. Crates on line 3, for example, will need to be pushed onto our `Dock`'s before the crates on line 2 in order to preserve proper first-in, last-out ordering.

I propose the following solution. 