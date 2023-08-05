# Day 4: Camp Cleanup
> **Key Concepts**
> `String::split`
> Boolean operators

In Day 4's challenge, we are given pairs of integer ranges and asked to find pairs with _completely_ (part 1) or _partially_ (part 2) overlapping ranges. Finding overlap is a simple matter of comparing integer sizes. The tricky part of this challenge, for me, was parsing the input, so we'll focus on that.

Let's whip this out.
```bash
# aoc
$ cargo new day_4
$ cd day_3

# add shared library
$ cargo add aoc --path ".."
```