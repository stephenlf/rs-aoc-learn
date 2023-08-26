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
enum TokenParserError {
    UnexpectedTokenError,
    BadArgumentError,
    MissingArgumentError,
}
```

### Presenting nice error messages to the user
Error messages are defined in our `Display` trait implementation. A good message should be short, concise, and provide useful information. To this end, I think we should encode our error variants into our error message in some way.

While we're at it, let's derive `Debug` and implement the `Error` trait as well.

```rust 
// aoc/day_10/src/lib.rs
// ..
#[derive(Debug)]
enum TokenParserError {
    // ..
}

impl std::fmt::Display for TokenParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            &Self::UnexpectedTokenError => 
                "unexpected token",
            &Self::BadArgumentError => 
                "bad argument; expected a whole number",
            &Self::MissingArgumentError => 
                "missing argument; expected a whole number",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for TokenParserError {}
```
It should be noted that even though we implement `Display` for our error, most end users of our library will only ever see the `Debug` message. Calling `Err(TokenParsingError::BadTokenError).unwrap()`, for example, will only provide the following output:
```bash 
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: MissingArgumentError'
```
