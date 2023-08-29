use day_10::*;
use aoc;
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
    let (width, height) = (40u16, 6u16);
    let image = fs::File::create(path).unwrap();
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    encoder
}

fn main() {
    let lines = aoc::read_as_lines("../inputs/day_10.txt").unwrap();

    let mut encoder = build_encoder("crt.gif");

    let mut clock = Clock::new();
    
    for line in lines {
        let line = line.unwrap();
        let token = Token::try_from(line).unwrap();
        match token {
            Token::Addx(n) => clock.addx(n),
            Token::Noop => clock.noop(),
        }
        clock.write_to_gif(&mut encoder);
    }
}