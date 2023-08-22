# Day 9: Rope Bridge
> **Key Concepts**
> - State machines with enums
For today's challenge, we are tasked with tracing the path of a two-unit length of rope being dragged by its head. We are going to solve this puzzle the worst way I know how: a state machine. The final product will be verbose, unmaintainable, and unextensible. But it will be a good demonstration of the power of Rust's enums in validating state machines at compile time.

This code will be so bad, in fact, that I invite you to _not_ follow along in your own editor. Instead, try to implement a solution that calculates changes in the absolute position of a tail, based on the change in relative position of the head. I will let you take care of that.

In the meantime, I will begin.