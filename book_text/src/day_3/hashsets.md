# The Hashset Collection
Rust's hashset can be thought of as a collection of unique items (called keys). Keys may be `insert`ed, `removed`ed, and otherwise manipulated as you might expect. Keys are stored according to a hashing algorithm, meaning lookup is incredibly fast (O(_n_) with respect to the length of the key _only_; O(_1_) with respect to the length of the set). However, this also means that elements are necessarily unsorted. `HashSet`'s are analogous to Python's `set`'s.

The `HashSet` is actually a narrow implementation of a broader structure, the `HashMap`, which stores key-value pairs hashed against their key. In that way, `HashMap` is somewhat analogous to Python's `dictionary`.

The reason we prefer `HashSet`'s over some other collection like `Vec` is for its quick `intersection` calculations. The implementation of the `intersection` function is fairly straitforward ([source](https://github.com/rust-lang/rust/blob/master/library/std/src/collections/hash/set.rs#L1611), line 1611). In essence, the implementation is equivalent to something like this: 

```rust 
    use std::collections::HashSet;

    let hs1 = HashSet::from([1,2,3]);
    let hs2 = HashSet::from([2,3,4]);

        // Iterate over first hashset
    let mut intersection = hs1.iter()       
        // Check if each element is in second hashset
        // In std::collections, this check is computed lazily.
        .filter(move |element| hs2.contains(element));   


    // Assert that `intersection` contains 2 and 3 and nothing else.
    let element = intersection.next().unwrap();
    assert!(*element == 2 || *element == 3);

    let element = intersection.next().unwrap();
    assert!(*element == 2 || *element == 3);

    assert!(intersection.next().is_none());
```
The big gains found in using `HashSet`'s in this implementation rather than `Vec`'s or something else lie in the `contains` function in line 10. Checking if a `Vec` contains some element requires iterating over the length of the vector until a match is found. This operation is O(_n_) with respect to the length of the vector. On the other hand, checking if a `HashSet` contains an element can be done in constant time with respect to the size of the set.

## Our implementation
To solve our puzzle, we will need to create a function that can do the following:
1) Split a string in half
2) Load each half into a different `HashSet`
3) Find the `intersection` between the two sets (we are assuming the intersection is a single `char`)
We can then turn the intersecting `char` into its appropriate integer score value and count up the total.

I encourage you to try to implement this function on your own. Once you do, come back and check out my hints below. There are a lot of ways to solve this problem, some reflecting a deeper understanding of Rust than others. If you can, try to iteratively improve your solution using the hints below. If you get stuck, you can always check out my solutions in the source code.

> **HINTS**
> > You can cast a character with `char as u32` or other integer, returning the [ASCII value](https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html) of the character. Conversion into a _priority_ value according to the challenge prompt should be simple from there.
>
> > Remember to use our **shared library**'s `read_as_lines` function to iterate over the lines of the input.
>
> > You can `map` over the lines of the input, turning each line into its appropriate intersection `char`, and then into a _priority_ and calling `std::iter::Iter::sum` at then end. Alternatively, you can call `fold`, which operates like a `map`, but takes in an accumulator variable which can hold a running total of your _priority_ score.
> > ```rust
> > let l = ['a','b','c']; 
> > let sum_1: u32 = l.iter().map(|c: &char| *c as u32).sum();
> > let sum_2: u32 = l.iter().fold(0, |total: u32, c: &char| total + (*c as u32));
> > assert_eq!(sum_1, sum_2);
> > ``````