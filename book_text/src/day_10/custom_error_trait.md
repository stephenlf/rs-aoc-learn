# Parsing with a Custom Error
One frequent pattern that has come up in our solutions has been _parsing our input_ into functions or data. String parsing is an error-prone process. There are no guarantees that our input strings are formatted correctly, or even accessible. 

So far, we have handled `Result<_>`'s (and thus potential errors) in one of two ways: handle it immediately with an `unwrap`, or propogate it with a string literal `Err(&'static str)`. [As discussed in the book](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html), both of these approaches are fine for simple programs like our own. However, for libraries or production-ready binaries, we need to do better. 

When errors show up in our libraries, it is typically best to propogate them with a custom error type. The minimum requirements for an `Error` type are as follows:

1) It implements `Debug` and `Display`
2) It implements `std::error::Error`.

Let's look at the definition of the `Error` trait in the standard library.

```rust
pub trait Error: Debug + Display {
    // Provided methods
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    fn description(&self) -> &str { ... }
    fn cause(&self) -> Option<&dyn Error> { ... }
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) { ... }
}
```
In the first line, we see that any `Error` type must also implement `Debug + Display`, like was noted above. In the trait body, we also see four "provided methods". These methods are derived automatically by the compiler for any object that implements the `std::error::Error` trait; we don't have to implement those functions ourselves. _We also won't be calling these functions, so we can safely ignore them entirely_.

In fact, once we've defined `Debug` and `Display` for our enum/struct `MyError`, implementing `Error` almost doesn't take any code at all! Check it out.

```rust
use std::error::Error;                  // <-- Bring Error trait into scope

#[derive(Debug)]                        // <-- Implement Debug
struct MyError;

impl std::fmt::Display for MyError {    // <-- Implement Display
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This was an error")
    }
}

impl std::error::Error for MyError {}   // <-- Implement Error. (No code!)
```
_Rust By Example_ has [guidelines](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html) for good custom error types. 

> _Rust By Example_
> - Represents different errors with the same type
> - Presents nice error messages to the user
> - Is easy to compare with other types
>   - Good: `Err(EmptyVec)`
>   - Bad: `Err("Please use a vector with at least one element".to_owned())`
> - Can hold information about the error
>   - Good: `Err(BadChar(c, position))`
>   - Bad: `Err("+ cannot be used here".to_owned())`
> - Composes well with other errors

Let's look at what a good error type might look like in the context of the puzzle. All of the code I include here will go in a library crate.

## Designing a good `Error` type
We can model our input as a stream of `Tokens`. There is an `addx` token, with an associated `i32` data member, and the `Noop` token, which has no associated data.

```rust
// aoc/day_10/src/lib.rs
{{ #include ../../../aoc/day_10/src/lib.rs:token}}
```

Let's implement a new function, `try_from::<String>`, which is defines a fallible (non-panicking) converter. We have already seen this usage before in previous chapters. `try_from` is a part of the `TryFrom::<T>` trait. 

```rust
// aoc/day_10/src/lib.rs
// ..
impl TryFrom::<String> for Token {
    type Error = todo!();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
```
The `TryFrom` trait has us declaring our own error type right off the bat. This type will be the type that we return when conversion fails. Previously, we defined that to be `type Error = &'static str`, which meant that we could simply return `Err("Error message here")` when an error occurred. Unfortunately, ([despite popping up in Rust documentation](https://doc.rust-lang.org/std/convert/trait.TryFrom.html#generic-implementations)) the string-slice error type is a bit of an antipattern.

Instead, let's define our own error type: `TokenParserError`. 

```rust
// aoc/day_10/src/lib.rs
// ..
struct TokenParserError;
```
Now let's try to turn this into a _good_ error type following the rules given by _Rust By Example_ above.

### Representing multiple errors with the same type
Parsing tokens can fail in a few different ways. We can get an unexpected token in our input:
```
addx 12
noop
remd        <-- Unexpected token!
addx -2
```
We can get a poorly formatted `addx` argument:
```
addx 12
addx 3.4    <-- Poorly formatted argument!
addx -2
```
Or we can be missing the `addx` argument:
```
addx 12
addx        <-- Missing argument!
addx -1
```
We can capture these failure types in an enum. Let's remove our error struct and replace it with an enum.
```rust
// aoc/day_10/src/lib.rs
// ..
pub enum TokenParserError {
    UnexpectedToken,
    BadArgument,
    MissingArgument,
}
```

### Presenting nice error messages to the user
Error messages are defined in our `Display` trait implementation. A good message should be short, concise, and provide useful information. To this end, I think we should encode our error variants into our error message in some way.

While we're at it, let's derive `Debug` and implement the `Error` trait as well.

```rust 
// aoc/day_10/src/lib.rs
// ..
#[derive(Debug)]
pub enum TokenParserError {
    // ..
}

impl std::fmt::Display for TokenParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            &Self::UnexpectedToken => 
                "unexpected token",
            &Self::BadArgument => 
                "bad argument; expected a whole number",
            &Self::MissingArgument => 
                "missing argument; expected a whole number",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for TokenParserError {}
```
It should be noted that even though we implement `Display` for our error, most end users of our library will only ever see the `Debug` message. Calling `Err(TokenParserError::BadArgument).unwrap()`, for example, will only provide the following output:
```bash 
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: BadArgument'
                                                              # ^ Debug message only
```
So it's important that our debug message is relatively descriptive as well. See [this thread](https://users.rust-lang.org/t/why-does-error-require-display-but-then-not-use-it/65273) for more details.

### Making errors composable with other errors and comparable to other types
I think we already have this aspect figured out. Our error type names `TokenParserError`, `BadArgument`, etc. are all descriptive enough and generally follow the patterns created by other libraries. This is a bit of a looser requirement, especially since different libraries have their own little spins on the error type. But, in general, I don't think anybody would need more than a cursory scan of a message containing our error type to know what's going on. 

It's also very clear that our `TokenParserError` type is an _**error**_--clarity that's not afforded to us when we use `&'static str` for our errors.

### Holding information about our error
Our `TokenParserError` doesn't yet capture any specific information about the errors it describes. We can fix that by attaching a data member to our variants. I can think of two instances where that might be appropriate:

1) Our `UnexpectedToken` error can store the token that triggered the error. 
2) Our `BadArgument` error can store the argument that triggered the error.
```rust
// aoc/day_10/src/lib.rs
// ..
pub enum TokenParserError {
    /// Captures the unexpected token that was supplied
    UnexpectedToken(String),
    /// Captures the bad argument that was supplied
    BadArgument(String),
    MissingArgument,
}
```
These items can be passed up in our error message, or referenced by our end users later. Let's update our `Display` trait to reflect this additional data.
```rust
// aoc/day_10/src/lib.rs
// ..
impl std::fmt::Display for TokenParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnexpectedToken(token) => 
                format!("unexpected token {token:?}"),
            Self::BadArgument(argument) => 
                format!("bad argument {argument:?}; expected a whole number"),
            Self::MissingArgument => 
                "missing argument; expected a whole number".to_string(),
        };
        write!(f, "{message}")
    }
}
```
## Bringing it together
I think we have a solid error type defined. It's clear, comparable, and composable. It plays nicely with the type system, and carries useful information. Let's look at it again to enjoy its beauty.
```rust
// aoc/day_10/src/lib.rs
// ..

#[derive(Debug)]
/// Error that may be thrown while parsing commands
pub enum TokenParserError {
    /// Captures the unexpected token that was supplied
    UnexpectedToken(String),
    /// Captures the bad argument that was supplied
    BadArgument(String),
    MissingArgument,
}

impl std::fmt::Display for TokenParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnexpectedToken(token) => 
                format!("unexpected token {token:?}"),
            Self::BadArgument(argument) => 
                format!("bad argument {argument:?}; expected a whole number"),
            Self::MissingArgument => 
                "missing argument; expected a whole number".to_string(),
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for TokenParserError {}
```
_Lovely_. But wait, why did we make an error type in the first place?
> ```rust
> // aoc/day_10/src/lib.rs
> // ..
> impl TryFrom::<String> for Token {/*..*/}
> ```
That's right! We needed some error type to implement `TryFrom` in our parser. Let's drop our new `TokenParserError` into the trait's `Error` type alias definition.
```rust
// aoc/day_10/src/lib.rs
// ..
impl TryFrom::<String> for Token {
    type Error = TokenParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
```
Now we just have to set up our logic! For my implementation, I'll first split the input string at whitespace, then match for the patterns I expect to be present. For all other patterns, I will throw the appropriate error. Here's what that looks like:
```rust
// aoc/day_10/src/lib.rs
// ..
{{ #include ../../../aoc/day_10/src/lib.rs:tryfrom}}
// ..
```
Wonderful! We now have a robust, idiomatic way to parse our inputs.

There's nothing new that can be learned about Rust in solving the rest of this puzzle, so I will leave that to you. Check out the source code for my full solution. Instead, let's see if we can't make a pretty picture of our outputs.