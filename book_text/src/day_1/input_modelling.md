# Day 1: Calorie Counting

> Key concepts: 
> - Data modelling with structs
> - Methods and derived traits
> - File IO with `BufReader`
> - `String::parse<T>`
> - Unit tests

Our first challenge is rather simple. We are given a list of integers (Calorie counts) and told to find the elf who is carrying the most calories. To tackle this, we will first be modelling our data as structs and data members. We'll then create methods that work on these structs to parse and load our input into memory. Finally, we'll sort the data to solve the challenge. Along the way, we'll create a unit test suite to validate our code's progress.

To begin, create a new package in our workspace.

```bash
# aoc/
cargo new day_1
cd day_1
```