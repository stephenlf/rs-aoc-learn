# File Parsing and IO: Pt. II
In this problem, we are given inputs of the following form,
```
1-2,3-4
...
```
where each line represents a pair of inclusive integer ranges. 

What information do we need to pull from this string? To me, it makes sense to pull out the min and max values of each range. To do that, we'll use the `String::split` and `String::split_at` functions. Take a look at their function signatures.

```rust
/// An iterator over substrings of this string slice, separated by characters matched by a pattern.
/// The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> {/*...*/}

/// Divide one string slice into two at an index.
fn split_once<'a, P>(&'a self, delimiter: P) -> Option<(&'a str, &'a str)> {/* ... */}
```

Notice the two different return types of these two functions. `split` returns an iterator, whereas `split_once` (which can only split a string once) returns a pair of `&str`.

Let's use these two functions to parse a line into a homemade `Range` structs, which holds a pair of integers. First we'll start by turning a single range into a `Range`.

```rust
// aoc/day_4/src/main.rs

/// Stores start and end of input range, inclusive
struct Range(u32, u32);

impl TryFrom<String> for Range {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {        
        let range = value.split_once('-');
        let (min, max) = if range.is_some() {
            range.unwrap()
        } else {
            return Err("Could not find right '-' token".into());
        };

        let min = min.parse::<u32>()?;    
        let max = max.parse::<u32>()?;

        Ok(Self(min, max))
    }
}
```
We should probably run some tests to make sure this function is working.
```rust
// aoc/day_4/src/main.rs
#[derive(PartialEq, Eq, Debug)]
struct Range(u32, u32);

// ..

#[cfg(test)]
mod day_4 {
    use super::*;

    #[test]
    fn test_range_from_string() {
        let test_string = String::from("1-2");
        let test_range = Range::try_from(test_string).unwrap();
        assert_eq!(test_range, Range(1, 2));
        
        // We should be prepared for multi-digit numbers
        let test_string = String::from("10-20");
        let test_range = Range::try_from(test_string).unwrap();
        assert_eq!(test_range, Range(10, 20));
    }
}
```

Looks good! With that done, we'll need to write a function that can split a full line and return two `Range`'s. I will let you implement that (it's not much different that the first parser we wrote).

With that done, all that's left to do is to compare the min and max values of each range and figure out their overlap. I solved this puzzle with conditional statements (see **HINTS**). I'm embarrassed to admit that it took _way_ longer than necessary to get the logic right. Once I simplified my logic and worked in some simple tests, I was able to get it. Go figure.

Good luck! And as always, full solutions in the source code.

> **HINTS**
> > The following is a snippet of my _total overlap_ checker
> > ```rust
> > if left_range.0 <= right_range.0 && left_range.1 >= right_range.1 {
> >     /* then left contains right... */
> > }
> > ```