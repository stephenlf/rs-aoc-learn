# Pattern Matching with Enums

A game of Rock Paper Scissors gives us a great opportunity to use Rust's powerful pattern matching tools. We can think of the hand we throw as a single type with three variants, and the outcome of the game as the result of a function on your hand and your opponents hand. Types with variants are defined with `enum`s:
```rust
// aoc/day_2/src/main.rs
enum Hand {
    Rock,
    Paper,
    Scissors,
}
```
Using an `enum` allows us to take advantage of one of Rust's superpowers: pattern-matching `match` blocks with compiler validation. Consider the following incomplete function:
```rust
fn game(my_hand: &Hand, opp_hand: &Hand) -> u32 {
    match (my_hand, opp_hand) {
        todo!()
    }
}
```
This code takes in the `Hand` that I and my opponent threw during a round of Rock Paper Scissors and returns the points (as `u32`) that I won that round. In the second line, `(my_hand, opp_hand)` creates a _pattern_ that our `match` block will match against. This pattern, denoted generally as `(Hand, Hand)`, can take forms such as `(Rock, Rock)`, `(Rock, Paper)`, `(Scissors, Rock)`, etc. 

The beauty of Rust's match block is that the `rustc` _will not_ compile unless every pattern is accounted for. With Rust, it's impossible for us to forget a branch, as long as we've modelled our states well.

Let's add the `game` function into our `main.rs` and start working in some business logic. In this problem, we get points both for the hand we throw (_hand points_) and whether or not we won (_win points_). For now, let's only calculate the win points. We will calculate hand points later. 

```rust
// aoc/day_2/src/main.rs
// ..
fn game(my_hand: &Hand, opp_hand: &Hand) -> u32 {
    use Hand::*;            // Lets us write `Rock` instead of `Hand::Rock`, etc.
    match (my_hand, opp_hand) {
        (Rock, Scissors) | (Paper, Rock)     | (Scissors, Paper)    => 6,  // win
        (Rock, Rock)     | (Paper, Paper)    | (Scissors, Scissors) => 3,  // tie
        (Rock, Paper)    | (Paper, Scissors) | (Scissors, Rock)     => 0,  // loss
    }
}
```
Looks nice! 

What's the best way to incorporate hand points into this function?

One approach would be to simply extend our `match` block's logic to take our hand points into consideration. Rather than our `match` block having three arms for "win," "tie," and "loss," we could give the block nine arms for "win with rock," "win with paper," "win with scissors," etc. This would be fine. However, the code would be unruly in its verbosity and difficult to maintain. I think there is a better way.

Each variant of an enum can hold a single piece of data. The data can be a simple primitive, like `u32` or `book`, or it can be of a more complex type like a `struct` or a `tuple`. With some care, it can even hold nested or recursive types. The data type held by each variant need not be the same; e.g. one variant can hold an integer while another holds a tuple, struct, or nothing at all.

Let's refactor our `Hand` variants to hold an integer. This integer will represent the value of the _hand points_ associated with each hand.
```rust
// aoc/day_2/src/main.rs
enum Hand {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}
```
Now in our `match` block, we can pull out the contents of each variant's data by assigning it to a variable `n`, like so:
```rust
// aoc/day_2/src/main.rs
// ..
fn game(my_hand: &Hand, opp_hand: &Hand) -> u32 {
    use Hand::*;            // Lets us write `Rock` instead of `Hand::Rock`, etc.
    match (my_hand, opp_hand) {
        (Rock(n), Scissors(_)) | (Paper(n), Rock(_))     | (Scissors(n), Paper(_))    => 6 + n,  // win
        (Rock(n), Rock(_))     | (Paper(n), Paper(_))    | (Scissors(n), Scissors(_)) => 3 + n,  // tie
        (Rock(n), Paper(_))    | (Paper(n), Scissors(_)) | (Scissors(n), Rock(_))     => 0 + n,  // loss
    }
}
```
With that complete, we can write our test cases and move on. I will let you write your own tests, but feel free to check out the source code to see what I wrote. 