use gif::{Encoder, Frame};
use std::borrow::Cow;

static mut FRAME_BUFFER: [u8; 40 * 7] = [0; 40 * 7];

pub struct Clock{
    /// Value of register X
    x: i32,
    /// Cycle number
    cycle: i32,
    /// Sum of signal strength (part 1)
    total: i32,
    /// CRT output (part 2)
    pub buffer: String,
}

impl Clock {
    pub fn new() -> Self {
        Self { x: 1, cycle: 0, total: 0, buffer: String::new() }
    }

    pub fn noop(&mut self) {
        self.inc_cycle();
    }

    pub fn addx(&mut self, n: i32) {
        self.inc_cycle();
        self.inc_cycle();
        self.x += n;
    }

    fn inc_cycle(&mut self) {
        // Write to buffer for part 2
        if ((self.cycle % 40) - self.x).abs() < 2 {
            self.buffer.push('#');
        } else {
            self.buffer.push('.');
        }
        
        self.cycle += 1;

        // Calculate part 1
        if (self.cycle - 20) % 40 == 0 && self.cycle < 221 {
            self.total += self.x * self.cycle;
        }
    }

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

//// ANCHOR: token
pub enum Token {
    Addx(i32),
    Noop,
}
//// ANCHOR_END: token

//// ANCHOR: tryfrom
impl TryFrom::<String> for Token {
    type Error = TokenParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split_whitespace().collect::<Vec<&str>>();
        match tokens.as_slice() {
            ["addx", arg] => {
                if let Ok(i) = arg.parse::<i32>() {
                    // Good input. `addx -2`
                    Ok(Self::Addx(i))
                } else {
                    // Bad input. `addx jkl`
                    Err(Self::Error::BadArgument(arg.to_string()))
                }
            }

            // Good input. `noop`
            ["noop"] => Ok(Self::Noop),

            // Bad input. `addx` (with no argument)
            ["addx"] => Err(Self::Error::MissingArgument),

            // All other patterns are bad inputs
            other => {
                let recollection = other.join(" ");
                Err(Self::Error::UnexpectedToken(recollection))
            },
        }
    }
}
//// ANCHOR_END: tryfrom

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