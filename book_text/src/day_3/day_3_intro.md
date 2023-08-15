# Day 3: Rucksack Reorganization
> **Key Concepts**
> - `HashSet`, a collection of unique values with O(1) access
> - `char` as integer
> - `Iterator::fold`
For today's challenge, we are tasked with splitting strings in half and finding the common character between them. Each string half can be modelled as a _[set](https://en.wikipedia.org/wiki/Set_(mathematics))_ of characters, and we need to find the [intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory)) of that set. 

Fortunately, Rust offers us the `HashSet` type with an available `intersection` method. We will take advantage of this functionality to solve today's challenge.

Since we've already covered a lot of the setup for these challenge problems, this tutorial will be shorter than the previous two.

Let's get started.
```bash
# aoc/
$ cargo new day_3
$ cd day_3

# add shared library
$ cargo add aoc --path ".."
```