# Day 2: Rock, Paper, Scissors
> **Key Concepts:**
> - Parsing token variants to enums
> - Match blocks
> - Setup methods in test blocks
> - `String::chars`

In this challenge, we are presented with a list of character pairs representing rounds of a Rock, Paper, Scissors game. We are told that we will score points based on 1) the Rock/Paper/Scissors variant we choose, and 2) whether we won, tied, or lost. We are then asked to calculate the total score we are expected to get by the end of all these rounds. 

Sounds easy.

```bash
# aoc
$ cargo new day_2
$ cd day_2

# add shared library
$ cargo add aoc --path ".."
```