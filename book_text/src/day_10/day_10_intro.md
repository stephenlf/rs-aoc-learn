# Day 10: Cathode-Ray Tube
> **Key Concepts**
> - The `Error` trait
> - The `gif` crate
There is very little required to solve Day 10 that we haven't already covered in previous chapters. We parse the input, save a value, do some math on it, etc. Even part 2 has little to offer, as we're just updating a 2D vector much like we did in Day 8. There is one concept that we've glossed over in previous solutions: error handling. To kick off this chapter, we'll implement a custom `ParsingError` following [best practices](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html).

Part 2 has us creating a virtual [CRT screen output](https://en.wikipedia.org/wiki/Cathode-ray_tube). We can simulate the screen with an ASCII output, like the prompt did. But, just for fun, I'm going to introduce the community crate `gif` and create a cute little animation to go along with our CRT simulation. 

```bash
# aoc
$ cargo new day_10
$ cd day_10

# add shared library
$ cargo add aoc --path ".."
```