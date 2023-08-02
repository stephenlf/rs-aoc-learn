# The ```TryFrom``` Trait
With our game-scoring function prepared, we now need to focus our attention on parsing and decoding our input. There are a couple of ways we can approach this. We can define a self-contained function that takes in a `char` and outputs a `Hand`, like so: 
```rust 
fn parse_hand_from_char(c: char) -> Hand {...}
```
We can also drop the above function in a bespoke `impl` block.
```rust
impl Hand {
    fn from_char(c: char) -> Self {...}
}
```
However, Rust already provides a trait method with this exact functionality: the `TryFrom` method [[doc](https://doc.rust-lang.org/std/convert/trait.From.html)]. Let's look at the trait definition from `std::convert::TryFrom`.
```rust
pub trait TryFrom<T>: Sized {
    type Error;

    // Required method
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```
This trait provides a single method, `try_from`, which converts an object of one type into another type, consuming the original object. By implementing `TryFrom<char>` for our `Hand` enum, we will be able to easily parse our problem's input. 

In general, it is best to always favor trait implementations over bespoke methods. Much of the code in the standard library operates on traits, meaning our code can slot in seamlessly with the standard library if we implement traits. As an example, by using the `TryFrom` trait, the compiler can implicitly derive the reverse function `TryInto::<char>::try_into`. That's two functions for the price of one! Furthermore, many Rustaceans look to trait implementations first when they want a particular functionality from their APIs. Implementing traits for your structures will make your code easy to use and idiomatic.

Let's look at what an implementation of `TryFrom` on our `Hand` enum might look like.

```rust 
// aoc/day_2/std/main.rs
// ..
impl TryFrom<char> for Hand {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Self::Rock(1)),
            'B' | 'Y' => Ok(Self::Paper(2)),
            'C' | 'Z' => Ok(Self::Scissors(3)),
            _ => Err("Unexpected character"),
        }
    }
}
```
First, we change the generic type parameter `T` to `char`, to declare that we're converting from a `char`. Next, we define what the `Error` type of our function will be. This can be any type that implements the `std::error::Error` trait. In our case, our `Error` type will just be a string literal. 

In our function's body, we create another match block that takes in one `char` of either A-C or X-Z and returns the appropriate `Hand` variant. It also embeds the proper _hand points_ into the returned value. 

> I hope you can see how advantageous it was to encode the _hand points_ within the `Hand` enum. Because we did that, we only need to account for the _hand points_ of each variant once at initialization. Our code is simpler to write, simpler to read, and easier to maintain for it.

> **Warning: Error Types**
> In our `TryFrom` implementation above, we used `&'static str` as our error type. This is fine for our challenge problem. However, when designing production APIs, there are best practices to creating error types, and using raw string literals is an antipattern. 
>
> We will define our own `Error` type in a later chapter. For now, you can read _Rust By Example_'s [discussion](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html) of the issue.

We should probably define a few tests for our implentation, both to get a handle on the `try_from` syntax and to check our logic. In tests, the `assert_eq` macro implicitly calls the `==` operator on whatever we feed to it. Right now, using a `Hand` object in a `==` expression will result in a compiler error, since our type doesn't implement the equality/partial equality traits. That's simple enough to fix with a `derive` statement. The `assert` macros also require us to implement the `Debug` trait.
```rust
// aoc/day_2/src/main.rs
#[derive(PartialEq, Eq, Debug)]
enum Hand {...}

// ..

mod day_2_tests {
    #[test]
    fn test_from_char() {
        assert_eq!(Hand::try_from('A'), Ok(Hand::Rock(1)));
        assert_eq!(Hand::try_from('B'), Ok(Hand::Paper(2)));
        assert_eq!(Hand::try_from('C'), Ok(Hand::Scissors(3)));
        assert!(Hand::try_from('h').is_err());
        // etc...
    }
}
```

With `TryFrom<char>` implemented for our `Hand` enum, we are ready to start parsing our input. 