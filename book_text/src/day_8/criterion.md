# Benchmarking with Criterion
> _Criterion.rs helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately. You can optimize with confidence, knowing how each change affects the performance of your code._
>
> _[See Criterion on crates.io](https://crates.io/crates/criterion)_

We will be using the Criterion crate for our benchmarking. Criterion will run our target functions thousands of times to get a solid performance profile, perform statistical analysis, and generate cute visuals, all with minimal effort from a `cargo`-based CLI.

Let's follow the [quickstart guide](https://crates.io/crates/criterion#quickstart) on crates.io to set up our benchmarks.

First, we add `criterion` to our crate's _dev dependencies_.
```bash
# aoc/day_8
$ cargo add criterion --dev -F html_reports
```
Next, we define a `[[bench]]` section in our _cargo.toml_ manifest.
```toml
# aoc/day_8/cargo.toml
# ..
[[bench]]
name = "threading"
harness = false
```
This prompts cargo to look for benchmarking code in _aoc/day\_8/benches/threading.rs_. The `harness = false` item tells cargo to abandon its default `cargo test` behavior in favor of Criterion's.

To make our code available to Criterion, it needs to be in a lib crate. Lets define a new lib crate in _aoc/day\_8/src/lib.rs_ and copy the contents of _forest.rs_ to it. 

```bash
# aoc/day_8
cp src/forest.rs src/lib.rs
```

We should also [inline](https://nnethercote.github.io/perf-book/inlining.html) our target code. This will provide a more accurate representation of our code's performance. We will need to mark our `Forest::calc_*` functions `#[inline]`, as well as every supporting function that those functions call.

```rust
// aoc/day_8/src/lib.rs
// ..
impl Forest {
    // ..
    #[inline]
    pub fn calc_visibility(&mut self) {/* .. */}

    // ..
    #[inline]
    pub fn calc_multi(&mut self) {/* .. */}

    #[inline]
    fn scan_from(
        direction: Direction, 
        tree_grid: Arc<Vec<Vec<i32>>>, 
        tx: mpsc::Sender<(usize, usize)>
    ) {/* .. */}
}
// ..
```

With that in place, we can set up our Criterion benchmarks. This book isn't meant to be an exhaustive tutorial on Criterion, so we won't cover each component in detail. Please see the crate's [getting started guide](https://bheisler.github.io/criterion.rs/book/getting_started.html) for more details.

```rust
// aoc/day_8/benches/threading.rs
use criterion::{criterion_group, criterion_main, Criterion};
use day_8::Forest;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut forest = Forest::new("../inputs/day_8.txt");
    c.bench_function("single-threaded", 
        |b| b.iter(|| forest.calc_visibility())
    );

    let mut forest = Forest::new("../inputs/day_8.txt");
    c.bench_function("multi-threaded", 
        |b| b.iter(|| forest.calc_multi())
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```
This code defines two benchmarks: `"single-threaded"` and `"multi-threaded"`. After initializing a new `Forest`, the benchmarks run `forest.calc_visibility` or `forest.calc_multi` until enough measurements have been collected for a meaningful analysis. Running `cargo bench --verbose` shows the following output:

```bash
# aoc/day_8
$ cargo bench --verbose
# ..

Benchmarking single-threaded: Warming up for 3.0000 s
benchmarking single-threaded: Collecting 100 samples in estimated 5.1213 s (101k iterations)
single-threaded         time:   [50.027 µs 50.337 µs 50.683 µs]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

Benchmarking multi-threaded: Warming up for 3.0000 s
Benchmarking multi-threaded: Collecting 100 samples in estimated 5.6704 s (10k iterations)
multi-threaded          time:   [419.57 µs 425.49 µs 433.87 µs]
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe
```
The output shows us the mean runtime of our benchmarks (the middle `time` listed), as well as the ±95% confidence interval. To calculate the confidence interval, Criterion took 100 samples in ~5 seconds for each benchmark. Each sample has a varying number of iterations, or repetitions of the benchmark functions. In all, Criterion was able to run the `forest.calc_visibility` 101k times, and `forest.calc_multi` 10k times! That's a solid sample size!

Criterion will also generate an HTML report and save it to _aoc/target/criterion/report_. You can see what my report looks like [here](./report.html). Isn't it beautiful?

So how does our shiny, multithreaded algorithm stack up against the small-brained, single-threaded solution? Let's look back at that output from `cargo bench`

```bash
single-threaded         time:   [50.027 µs 50.337 µs 50.683 µs]
# ..
multi-threaded          time:   [419.57 µs 425.49 µs 433.87 µs]
```
On average, our single-threaded function took ~50 µs to complete, whereas our multithreaded solution took ~425 µs to complete. That's an _**8.5x decrease in efficiency**_. Amazing! Our multithreaded solution is terrible! If we keep contriving these overly complicated, slow solutions, we'll be well on our way to becoming a [-10x developer](https://taylor.town/-10x).

That's a joke, of course. Learning your language will only help you on your programming journey. _And have we not learned?_

Excellent work. Let's move on.