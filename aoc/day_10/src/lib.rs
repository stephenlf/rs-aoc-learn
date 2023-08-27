
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