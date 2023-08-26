
//// ANCHOR: token
enum Token {
    Addx(i32),
    Noop,
}
//// ANCHOR_END: token


impl TryFrom::<String> for Token {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub enum TokenParserError {
    UnexpectedTokenError,
    BadArgumentError,
    MissingArgumentError,
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