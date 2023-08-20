# A Single-Threaded Solution
Our challenge input consists of a 2D grid of trees. Each tree can be thought of having two data members: `height` and `visibility`. Our `Forest`, then, will be a 2D vector of `Tree`s. Let's write that out. Since we expect there to be a lot of code, let's put our code in a `forest` module.

```rust
// aoc/day_8/src/main.rs
mod forest;

fn main() {}
```
```rust
// aoc/day_8/src/forest.rs      <---DIFFERENT

struct Forest(Vec<Vec<Tree>>);

struct Tree {
    height: i32,
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
    fn new(path: &'static str) -> Self {
        let lines = aoc::read_as_lines(path).unwrap();

        let array = lines.map(|line| {

            let line = line.unwrap();
            
            line.chars().map(|c| {
                let height = c.to_digit(10).expect("Int parsing error");
                Tree::new(height as i32)
            })
            .collect::<Vec<Tree>>()

        })
        .collect::<Vec<Vec<Tree>>>();

        Self(array)
    }
}
// ..

impl Tree {
    fn new(height: u32) -> Self {
        Self {height, visibility: false}
    }
}
```

We now just need to figure out which trees are visible and update them accordingly. We can do this by moving from left to right across each row of our `Forest`, marking each `Tree` "visible" if it's at least as tall as any previous tree. We then repeat this from top to bottom, left to right, and bottom to top. 

To get this done, we'll need a way to grab our `Tree`'s. We can access points on our grid much like we would coordinates on a coordinate plane, using the following pattern.

`cell = grid[row_number][column_number]`

Note that unlike a coordinate plane, grid coordinates are accessed by row first, then column, like so:

```rust
//  vec_2d: Vec<Vec<char>>
//    0 1 2 3 4 5 6 7 8 9
//  0 . . . . . . . . . . 
//  1 . . . . . . . . . .
//  2 . . . . . . . x . .
//  3 . . . . . . . . . .
//  4 . . . . . . . . . .
//  5 . . . . . . . . . .

assert_eq!(vec_2d[2][7], 'x');

```
Note that this (row, column) pattern is arbitrary--merely a result of modeling our data as a `Vec` of rows, each containing a `Vec` of cells. We could also model the grid as a `Vec` of columns, each containing a `Vec` of cells. We choose the former model because our data is interpreted by line (row). 

With this pattern in mind, let's calculate each `Tree`'s visibility.

```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    // ..
    fn calc_visibility(&mut self) {
        let (rows, columns) = (self.0.len(), self.0[0].len());
        let mut max_height: i32;

        // Left to right
        for i in 0..rows {
            max_height = -1;
            for j in 0..columns {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }
        // ..
    }
}
// ..
```
The provided `calc_visibility` method will scan each row of trees from left to right, updating visibility as it goes along. As an exercise, try extending the `calc_visibility` method to update visibility from the other three directions. As always, check out the source code for a full solution.

With visibility calculated, we just need to sum up the number of visible `Tree`'s to complete our puzzle. 

## Flatten the map

Iterating over a 2D vector comes with some added complexity over a one dimensional iterator. Simply turning our `Forest` data member will return `Iter<Vec<Tree>>`, an iterator over vectors! To properly iterate over each tree, we would have to `map` each row's `Vec` to another iterator, creating a mess of nested `Iter`'s and `map`'s. 

Fortunately, Rust provides the `flatten` convenience method, which flattens an `Iter<Iter<_>>` into a single iterator. Here's one summation implementation that `flatten`'s our `Forest` into an iterator over `&Tree`, then `fold`'s that iterator into a single total representing the number of visible trees.

```rust
// aoc/day_8/src/forest.rs
// ..
impl Forest {
    // ..
    pub fn sum_visibility(&self) -> u32 {
        self.0.iter().flatten().fold(0, |accum, tree| {
            if tree.visibility {
                accum + 1
            } else {
                accum
            }
        })
    }
}
```
You can probably think of a couple of other ways to solve this. For example, you might `flatten` the `Forest`, `filter` for visible `Tree`'s, `collect` the iterator back into a `Vec`, and calculate its length. Or you might just use a `for` loop.

But none of this is what I really want to cover during today's challenge. No, today, we are going to do some concurrency. Let's see what parallel processing can do for us.