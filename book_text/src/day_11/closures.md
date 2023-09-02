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
    target_if_true = usize
    target_if_false = usize
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
    id: usize
    items: VecDeque<usize>
    operation: ??
    test: ??
    target_if_true = usize
    target_if_false = usize
}
```

## Closure Types