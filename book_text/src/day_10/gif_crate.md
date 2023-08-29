# The `gif` Crate

Part 2 of this puzzle has us simulating a CRT screen. Rather than simply writing to an ASCII string, let's make a visual like the one below. 

<details><summary>SPOILERS: Solution ahead</summary>
<img src="./crt.gif" style="width:200px;"/>
</details>


To power our gif maker, we'll be using the [`gif` crate](https://crates.io/crates/gif). 

Let's do this!

```bash
# aoc/day_10
$ cargo add gif
```
## Capturing state

For my solution to part 2 of the puzzle, I created a struct to hold the following data members:

```rust
struct Clock{
    /// Value of register X
    x: i32,
    /// Cycle number
    cycle: i32,
    /// Sum of signal strength (part 1)
    total: i32,
    /// CRT output (part 2)
    buffer: String,
}
```
At each instruction (line of input), I run the `addx` or `noop` instruction, as appropriate. The logic for these two operations is stored in methods of the `Clock` struct (not shown here; see source code). All together, my `main` function looks like this:

```rust
// aoc/day_10/src/main.rs
use day_10::*;
use aoc;

fn main() {
    let mut clock = Clock::new();
    
    for line in lines {
        let line = line.unwrap();
        let token = Token::try_from(line).unwrap();
        match token {
            Token::Addx(n) => clock.addx(n),
            Token::Noop => clock.noop(),
        }
    }

    // Part 1: Print out clock.total
    // Part 2: Print out clock.buffer
}
```

Storing the state of our `Clock` like this provides us with a nice way to visualize what's going on inside our system. After every clock cycle, we can step in an pull out whatever state information is useful for our visualization.

At a high level, creating a gif with the `gif` crate works as follows:
- Create a new `Encoder` and capture configuration (color, size, output file, etc.)
- Each frame, write pixel data to a `&[u8]` buffer
- Pass each pixel data buffer to the encoder to write to the output file.

Here's what that will look like in practice. First, we build our encoder.

```rust
// aoc/day_10/src/main.rs
use day_10::*;
use std::{fs, borrow::Cow};
use gif::{Encoder, Repeat};

fn build_encoder(path: &str) -> Encoder<fs::File> {
    // Encode colors as trios of (r, g, b) hex values
    let color_map = &[
        25, 25, 25,         // Color 0 = grey (4,4,4)
        0xFF, 0, 0,         // Color 1 = red  (255,0,0)
        0, 0xFF, 0,         // Color 2 = green(0,255,4)
        0xFF, 0xFF, 0xFF    // Color 3 = white(255,255,255)
    ];

    // I add a 7th row to show the register pointer
    let (width, height) = (40u16, 7u16);
    let image = fs::File::create(path).unwrap();
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    encoder
}

fn main() {
    let lines = aoc::read_as_lines("../inputs/day_10.txt").unwrap();

    let mut encoder = build_encoder("crt.gif");
    // ..
}
```
Next, we pass encode our clock's state into a buffer. The buffer will be a single array that's `40 * 7 = 280` items long. To save on memory, I will hold a single, `static` buffer outside of any function definitions. Altering this buffer will require `unsafe` code, but since we're only accessing it with one thread, we shouldn't get any undefined behavior.

This write function will be a method of our `Clock` struct.
```rust
// aoc/day_10/src/lib.rs
use gif::{Encoder, Frame};
use std::borrow::Cow;

static mut FRAME_BUFFER: [u8; 40 * 7] = [0; 40 * 7];

// ..

impl Clock {
    // ..
    fn update_buffer(&self) {
        // Clear buffer (set to grey) 
        unsafe { FRAME_BUFFER = [0; 40 * 7] }
                

        // Write visible cells as green
        for (i, c) in self.buffer.char_indices() {
            if c == '#' {
                unsafe { FRAME_BUFFER[i] = 2 };
            }
        }

        // Write clock number pointer as white
        unsafe { FRAME_BUFFER[self.cycle as usize] = 3 }

        // Write register X as three pixel wide, red dot on bottom row
        for i in self.x - 1 .. self.x + 1 {
            if i >= 0 && i < 40 {
                let dot = (i + (40 * 6)) as usize;
                unsafe { FRAME_BUFFER[dot] = 1 };
            }
        }
    }
}

// ..

```
Finally, we pass our buffer into a `gif::Frame` object and write it to our `encoder`.
```rust
// aoc/day_10/src/lib.rs
// ..
impl Clock {
    // ..
    pub fn write_to_gif(&self, encoder: &mut Encoder<std::fs::File>) {
        self.update_buffer();
        let frame = Frame {
            width: 40,
            height: 7,
            buffer: unsafe { Cow::Borrowed(&FRAME_BUFFER) },
            ..Default::default()
        };

        encoder.write_frame(&frame).unwrap();
    }
}
```
Now we can call `write_to_gif` after every clock cycle, and our program will build a little visualization for us. 

Neat!