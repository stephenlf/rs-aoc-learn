use day_10::*;

fn main() {
  let _: Result<(), TokenParserError> = Err(TokenParserError::BadArgument(String::from("s"))).unwrap();
}