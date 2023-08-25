# An Enum-Powered State Machine
We can model the motion of our rope's tail as a [state machine](https://en.wikipedia.org/wiki/Finite-state_machine). Our system's state would be the relative position of the head of our rope to the tail. Transitions between states would be the head's motion in any of the four cardinal directions. After each transition, we can calculate the new position of our rope's tail and store it in a hash set (thereby eliminating duplicate values). Once our puzzle input is processed, the length of the hashset is the answer to our puzzle.

With Rust's enums, we can greatly simplify developing correct state machines by [making invalid states unrepresentable](https://youtu.be/7GzQArrek7A). Let's first define a `StateMachine` struct which holds the relative position of the head (relative to the tail), the absolute position of the tail, and the set of all visited coordinates.
```rust
// aoc/day_9/src/main.rs
{{ #include ../../../aoc/day_9/src/main.rs:state_machine}}
```
Next, I will model the machine's valid **states** and **transitions** with enums. The states consist of all of the relative positions the rope's head can take (e.g. Up, Down, Center, UpLeft, etc.), and the transitions are each of the four directions the head can move to. 

First, the states.
```rust
// aoc/day_9/src/main.rs
{{ #include ../../../aoc/day_9/src/main.rs:states}}
```
And the transitions.
```rust
// aoc/day_9/src/main.rs
{{ #include ../../../aoc/day_9/src/main.rs:transitions}}
```
Let's consider the state machine I have made, conceptually. Each of the relative positions of the head is a separate state. Assuming the tail is at the center of our diagram...
```
. . .
. T .
. . .
```
...then our head can be in any of the 9 positions shown.
```
H . .
. . .   UpLeft
. . .
--
. H .
. . .   Up
. . .
--
. . .
. H .   Center
. . .
--
. . .
. . .   DownRight
. . H
--
etc...
```
From each of the nine states, we can apply one of four transitions by moving the head up, down, left, or right. The outcome will be a new relative head position, or state. We can see this on our diagram.
```
H . .
. . .   UpLeft
. . .
  Move Right
. H .
. . .   Up
. . .
  Move Up
. H .
. . .   Up
. . .
  Move Down
. . .
. H .   Center
. . .
```
Notice that moving the rope up when it was already at position `Up` didn't change its relative position.

With nine states and four transitions, we have 9 * 4 = 36 different state/transition pairs to consider. We can annotate each of these pairs with a single line, like so:
```
[Old State] -[Motion]-> [New State]

Up -Up-> Up
Up -Down-> Center
Up -Left-> UpLeft
Up -Right-> UpRight

etc...
```
Every time the head moves, the tail has an opportunity to move as well. We can record the relative change in the tail's position based on the motion of the head at the end of every transition.
```
[Old State] -[Motion]-> [New State], [Change in tail's position as (x, y)]

Up -Up-> Up          (0, +1)
Up -Down-> Center    (0, 0)
Up -Left-> UpLeft    (0, 0)
Up -Right-> UpRight, (0, 0)

etc...
```
With the relative change in the tail's position calculated, we can then calculate the final position of the tail after every transition by adding the x and y components of the relative shift to the tail's absolute position.

I think I can start putting this into code.

First, I'll define a `new` `StateMachine`. I will preload the machine's hashset with the starting position (0, 0).
```rust
// aoc/day_9/src/main.rs
// ..
impl StateMachine {
{{ #include ../../../aoc/day_9/src/main.rs:new}}
}
```
Next, I'll start throwing my state/transition pairs into a match statement. The state machine logic will be wrapped in a function `step`, which takes in a direction and updates `self.abs_t` and `self.relative_h` appropriately.

I'm going to include the whole function below, mostly as a demonstration of why I think this programming pattern is bad for this use case. But just know that _I didn't miss a single arm_. Neat, right?
```rust
// aoc/day_9/src/main.rs
// ..
impl StateMachine {
{{ #include ../../../aoc/day_9/src/main.rs:step}}
}
```
But seriously, writing this code gave me a headache. Let's move on.