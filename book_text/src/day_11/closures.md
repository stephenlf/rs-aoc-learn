# Closure Types and Double-Ended Queues
It will be helpful to create a `Monkey` model. This struct will hold their inventory of `items`, their `operation`, and their divisibility `test`. This will end up being a lot of code, so let's drop it in a _lib_ crate.

After each operation, every monkey will throw the object in their hand to some other monkey. The monkey they throw to is dependent on the result of the `test`, so we should capture some information about their target monkeys in this struct as well. There are a few ways to accomplish this, but our method will be to assign each monkey an `id`. In each `Monkey`, we can store the `id` of its target monkeys (`target_if_true` and `target_if_false`).

```rust
// aoc/day_11/lib.rs

pub struct Monkey {
    id: usize
    items: ??
    operation: ??
    test: ??
    target_if_true: usize
    target_if_false: usize
}
```

> **Why do we store monkey `id`'s instead of pointers to other monkeys?**
>
> Using pointers is non-trivial to do. In unsafe languages like C or `unsafe` Rust, using pointers incorrectly can cause _undefined behavior_. In safe Rust, pointers require smart pointers like `Rc<RefCell<_>>` which add syntactic overhead and complexity (see [Day 7](../day_7/day_7_intro.md) for an exploration of this pattern).

Looks good so far. But what type should we assign to `items`, `operation`, and `test`? 

## Double-Ended Queues
Our `items` member represents the monkey's inventory, which can be modelled as a list of integers. In order to model our monkeys' behavior, we need to be able to pull items from one end of the list (to inspect and throw them), and add items to "the **end** of the recipient monkey's list." 

Typical stack-based list structures can't quickly pull the first element from their list, since it typically requires moving every item after the first element back an index. However, a doubly-linked list [does offer that functionality](https://en.wikipedia.org/wiki/Double-ended_queue#Operations). In Rust, we can use the datatype `std::collections::VecDeque`, which provides the functions `pop_back`, `pop_front`, `puch_back`, and `push_front`.

Let's add that to our `Monkey` struct.

```rust
// aoc/day_11/lib.rs

use std::collections::VecDeque;

pub struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: ??,
    test: ??,
    target_if_true: usize,
    target_if_false: usize,
}
```

## Closure Types
Our `Monkey`s' `operation` and `test` data members both encode _functions_. But how can we encode a function from our input?

One way would be to capture the unique parameters of each monkey's `operation` and `test`, then reconstruct the appropriate function at runtime. An incomplete implementation might look something like this:

```rust
struct DivisibilityTestMonkey {
    divisor: usize
    monkey_if_true: usize
    monkey_if_false: usize
}

impl DivisibilityTestMonkey {
    /// Takes in an item value and returns target monkey based
    /// on the items divisibility against self.divisor
    fn divisibility_test(&self, item: usize) -> usize {
        if item % divisor == 0 {
            self.monkey_if_true
        } else {
            self.monkey_if_false
        }
    }
}
```
This approach works fine for the divisibility test. However, trying to encode an `operation` like this will start to get unruly. Given the following representation of each operation,
```
        operand   operator  operand
           V          V        V
new = [old | int] [ + | * ]   int
```
we would need to encode the first _operand_ (previous value or some integer), the _operator_ (addition or multiplication), and the second _operand_. Not crazy, but not beautiful, either. We may also anticipate some runtime cost as well. Every time we called our `operation`, our program would have to traverse a tree of possible functions, reconstructing the function as it goes. Now, perhaps CPU caching could make the runtime cost negligible. But there is a better way.

Logically, it makes sense to model the entire `operation` or `test` function as a single object. Rust's [closure types](https://doc.rust-lang.org/reference/types/closure.html) let us do that. Let's see what that looks like.

```rust
// aoc/day_11/lib.rs
// ..
pub struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,  // NEW
    test: Box<dyn Fn(usize) -> bool>,        // NEW
    target_if_true: usize,
    target_if_false: usize,
}
```
In our assignments, `Fn(usize) -> usize` defines the [immutable closure trait](https://doc.rust-lang.org/std/ops/trait.Fn.html). It can be read as "a function that takes a `usize` and returns a `usize`." Unline `FnMut` and `FnOnce`, the `Fn` trait does not mutate or take ownership of its input variables or environment. (Since `usize` implements `Copy`, we don't need to use an immutable reference in our definition.)

The `dyn` keyword specifies that our data members are [trait objects](https://doc.rust-lang.org/std/keyword.dyn.html), meaning they don't have a specific type. This is can be read as "some object of unknown type that implements trait `Fn(usize) -> usize`. 

Since the size of trait objects can't be known at compile time, they must be placed behind a reference or smart pointer. That's where `Box` comes in. In total, our `operation`'s and `test`'s type notations may be read as "a pointer to an anonymous functions that takes in a `usize` and returns a `usize` (or a `bool`, for `test`).

> Note that you've likely seen a similar pattern with error handling. We often use `Box<dyn Error>` to denote "some object that implements the `Error` type."

This looks good! Let's see what parsing will look like for these new types.

## Parsing Closures and VecDeques 

Let's create three methods to act as parsers into each one of our data members. Each method will correspond to a single line of our input.

```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    /// Pulls monkey.id from the first line of each monkey block the input
    fn parse_id(line: String) -> usize { todo!() }

    /// Creates a VecDeque populated with items in the second line of the block
    fn parse_items(line: String) -> VecDeque<usize> { todo!() }

    /// Creates a closure matching `operation`, the third line of the block
    fn parse_operation(line: String) -> Box<dyn Fn(usize) -> usize> { todo!() }

    /// Creates a closure matching `test`, the fourth line of the block
    fn parse_test(line: String) -> Box<dyn Fn(usize) -> bool> { todo!() }

    /// Pulls the id of the target monkey (if test passes) from the fifth line
    fn parse_true_monkey(line: String) -> usize { todo!() }
    
    /// Pulls the id of the target monkey (if test fails) from the fifth line
    fn parse_false_monkey(line: String) -> usize { todo!() }
}
```
The `parse_id`, `parse_true_monkey`, and `parse_false_monkey` implementations are trivial, requiring only that we pull a single integer from each line. Here's my implementation of `parse_true_monkey`:

```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    // ..
    fn parse_true_monkey(line: String) -> usize { 
        // Example input: "    If true: throw to monkey 1"
        assert_eq!(&line[..29], "    If true: throw to monkey ");
        (&line[29..]).parse().unwrap()
    }
    // ..
}
```

Creating a new `VecDeque` is also simple, since `VecDeque`'s syntax is very similar to `Vec`'s. In my implementation, I use `VecDeque`'s builtin `from_iter` constructor, though the same result can be achieved with a combination of `new` and `push_back`.

```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    // ..
    fn parse_items(line: String) -> VecDeque<usize> { 
        // Example input: "  Starting items: 66, 59, 64, 51"
        assert_eq!(&line[..18], "  Starting items: ");
        
        let item_iter = (&line[18..])
            .split(',')
            .map(|item| item.trim().parse::<usize>().unwrap());

        VecDeque::from_iter(item_iter)
    }
    // ..
}
```
Creating our closure types may be a bit daunting because we haven't seen it before. But it isn't anything too new. Consider the following closure, which checks for divisibility against 7.
```rust
let my_closure: impl Fn(usize) -> bool = {
    |x: usize| x % 7 == 0
};
```
> Note that the code `x % 7 == 0` is equivalent to `if x % 7 == 0 {true} else {false}`

We can parameterize this closure by assigning the divisor `7` to a variable _outside_ of the closure, the `move`ing that variable into the closure.
```rust
let divisor: usize = 7;
let my_closure = {
    move |x: usize| x % divisor == 0
};
```
> `move`ing the divisor into the closure transfers ownership of the variable to the closure, meaning we don't have to worry about the lifetime of `divisor` anymore.

With just a little refactoring and `String` logic, this pattern will serve as our `Monkey::parse_test` function.
```rust
// aoc/day_11/lib.rs
// ..
impl Monkey {
    // ..
    /// Creates a closure matching `test`, the fourth line of the block
    fn parse_test(line: String) -> Box<dyn Fn(usize) -> bool> { 
        // Example input: "  Test: divisible by 11"
        assert_eq!(&line[..21], "  Test: divisible by ");
        let divisor = (&line[21..]).parse::<usize>().unwrap();

        let closure = { move |x: usize| x % divisor == 0 };
        
        Box::new(closure)
    }
    // ..
}
```
As a syntactical note, we can now call this closure using `function()` syntax. With `Box`ed closures, the compiler will automatically derefence our closure for us, like so:
```rust
let input = String::from("  Test: divisible by 11");
let divisible_by_11 = Monkey::parse_test(input);

assert_eq!(divisible_by_11(22), true);
assert_eq!(divisible_by_11(23), false);

// Closures within an object may require braces to keep the compiler informed
let monkey = Monkey { test: divisible_by_11, /*..*/ }
assert_eq!((monkey.test)(22), true);
assert_eq!((monkey.test)(23), false);
```
The last thing left to parse is our `operation`. It can be done in much the same way as `test`, with some nuance about how to account for the different operators. I will leave that as an exercise for you, though I will leave my complete solution in the source code.

Creating a new monkey now just requires calling our parsers on each line of the input in the right order. That will again be left as an exercise.

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
And there we go. We can now calculate each monkey's touch number, multiply the two highest values, and get our answer. Easy.

