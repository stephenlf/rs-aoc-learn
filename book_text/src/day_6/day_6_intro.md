# Day 6: Tuning Trouble
> **KEY CONCEPTS**
> - Sliding window iterators

In today's challenge, we are given a string of characters and told to find the first group of four unique items. To solve this, we'll iterate over the string with a 4-character [sliding window](https://doc.rust-lang.org/std/slice/struct.Windows.html) and check each window individually.

Super easy. In fact, since the challenge input is only one line, we don't even need to import our root library. Let's begin.
```bash
# aoc/
$ cargo new day_6
$ cd day_6
```