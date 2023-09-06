# The Ordering Trait
## Solving the problem
With our monkeys initializer prepared, it's time to start throwing items! Let's create a new method, `throw_items`, which loops through the monkey's `items`, popping them off, mutating them, and sending them to a `Vec` along with each items' target monkey.

First we mutate each item in `items`.
```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    // ..
    fn // ..
    while let Some(item) = self.items.pop_front() {
        let item = (self.operation)(item) / 3;

    }
    // ..
}
```
Next we calculate which monkey to throw it to.
```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    // ..
    while let Some(item) = self.items.pop_front() {
        let item = (self.operation)(item) / 3;
        let target_monkey = if (self.test)(item) {
            self.target_if_true
        } else {
            self.target_if_false
        }
    }
    // ..
}
```
Finally, we package each item and target monkey together, throw them all in a `struct`, and return it.
```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    // ..

    /// A single turn. The output is given as a list of pairs of numbers 
    /// Vec<(usize, usize)> where item.0 is the target monkey and item.1
    /// is the item to add to the stack.
    pub fn throw_items(&mut self) -> Vec<(usize, usize)> {
        let mut thrown_items: Vec<(usize, usize)> = vec![];

        while let Some(item) = self.items.pop_front() {
            let item = (self.operation)(item) / 3;
            match (self.test)(item) {
                true => thrown_items.push((self.target_if_true, item)),
                false => thrown_items.push((self.target_if_false, item)),

            }
        }
        
        thrown_items
    }
    // ..
}
```
In our `main` function, we just need to loop through each monkey, running `throw_items` on each one before assigning the returned `items` to their respective monkeys after each turn. We then repeat that process 20 times for our 20 rounds.
```rust
// aoc/day_11/src/main.rs
use aoc;
use day_11::*;
use std::cell::RefCell;


fn main() {
    // Initialize monkeys
    let mut lines = aoc::read_as_lines("../inputs/day_11.txt").unwrap();

    let mut monkeys: Vec<Monkey> = vec![];
    while let Some(monkey) = Monkey::new(&mut lines) {
        monkeys.push(monkey);
    }

    // Simulate rounds 1-20
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let items = monkey.throw_items();
            for (monkey_id, item) in items {
                monkeys[monkey_id].items.push_back(item);
            }
        }
    }
}
```
Except uh-oh! This won't compile! The borrow checker doesn't like that we're popping from one item in `monkeys` and pushing to another item in `monkeys` within a single loop. That breaks the single mutable reference rule. Fortunately, we can use a `RefCell` to keep the borrow checker happy.
```rust
// aoc/day_11/src/main.rs
// ..
fn main() {
    // Initialize monkeys
    let mut lines = aoc::read_as_lines("../inputs/day_11.txt").unwrap();

    // In our loop of rounds, we will need to pull from one monkey and push to 
    // another, all without breaking the loop. The borrow checker won't let us
    // do that with monkeys stored in Vec<Monkey>, so we wrap them in RefCell
    // to allow for run-time borrow checks.
    let mut monkeys: Vec<RefCell<Monkey>> = vec![];
    while let Some(monkey) = Monkey::new(&mut lines) {
        monkeys.push(RefCell::new(monkey));
    }

    // Simulate rounds 1-20
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let items = monkey.borrow_mut().throw_items();
            for (monkey_id, item) in items {
                monkeys[monkey_id].borrow_mut().items.push_back(item);
            }
        }
    }
}
```
Very nice! We now have a program that completes 20 rounds of monkey business. But hold on, we're missing the most vital piece of information, _how many touches each monkey makes_. That's this prompt's question!

No matter. To track the number of touches, we just need to add a counter to our `throw_items` method.
```rust
// aoc/day_11/src/lib.rs
// ..
pub struct Monkey {
    // ..
    pub touch_counter: usize,   // NEW
}

impl Monkey {
    // ..
    pub fn throw_items(&mut self) -> Vec<(usize, usize)> {
        // ..
        while let Some(item) = self.items.pop_front() {
            self.touch_counter += 1;                        // NEW
            // ..
        }
        // ..
    }
    // ..
}
```
And there we go. We can now print out each monkey's touch number, multiply the two highest values, and get our answer. Easy.

## The Ordering Trait
Something doesn't sit right with me. We've found the number of touches each monkeys make, and our input is simple enough that we could find the monkeys with the most touches pretty easily. But what if we had more monkeys? How can we compare and sort our vector of `Monkey`s?

Enter the [`Ordering` trait]().