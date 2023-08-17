# A Single-Threaded Solution
Our challenge input consists of a 2D grid of trees. Each tree can be thought of having two data members: `height` and `visibility`. Our `Forest`, then, will be a 2D vector of `Tree`s. Let's write that out. Since we expect there to be a lot of code, let's put our code in a `forest` module.

```rust
// aoc/day_8/src/main.rs
use aoc::*;

mod forest;
```
```rust
// aoc/day_8/src/forest.rs      <---DIFFERENT

struct Forest(Vec<Vec<Tree>>);

struct Tree {
    height: usize,
    visibility: bool,
}
```
We can write a pretty slick parser using `map` and `collect`. The parser will:
1) Split the input into `Line`'s
2) Split the `Line`'s into `char`'s
3) Parse the `char`'s as integers, then into `Tree`'s
4) Collect the `Tree`'s into `Vec<Tree>`
5) Collect the `Line`'s into `Vec<Vec<Tree>>`, then into `Forest`
Let's check it out.

```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    fn new(path: &'static str) {
        let lines = aoc::read_as_lines(path).unwrap();

        let array = lines.map(|line| {

            let line = line.unwrap();
            
            line.chars().map(|c| {
                let height = c.to_digit(10).expect("Int parsing error");
                Tree::new(height)
            })
            .collect::<Vec<Tree>>()

        })
        .collect::<Vec<Vec<Tree>>>();
    }
}
// ..

impl Tree {
    fn new(height: u32) -> Self {
        Self {height, visibility: false}
    }
}
```