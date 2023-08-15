/// Enumeration of possible instructions in each line of input
/// Tokens correspond to the given filesystem APIs.
pub enum Token {
    /// Filesystem::cd_root();
    CdRoot,
    /// Filesystem::cd_parent();
    CdParent,
    /// Filesystem::cd_child(child_name);
    CdChild(String),
    /// Ignored during parsing
    Ls,
    /// Filesystem::ls_dir(directory_name);
    Dir(String),
    /// Filesystem::ls_file(file_name, file_size);
    File(String, usize),
}

impl TryFrom<String> for Token {
    type Error = TokenParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = value.split_whitespace().collect();
        match tokens.as_slice() {
            &["$", "cd", "/"] => Ok(Self::CdRoot),
            &["$", "cd", ".."] => Ok(Self::CdParent),
            &["$", "cd", child] => Ok(Self::CdChild(child.to_string())),
            &["$", "ls"] => Ok(Self::Ls),
            &["dir", dir] => Ok(Self::Dir(dir.to_string())),
            &[size, child] => {
                let child = child.to_string();
                let size = size.parse::<usize>();
                if size.is_err() {
                    println!("{:?}", tokens);
                    return Err(TokenParserError);
                }
                let size = size.unwrap();
                Ok(Self::File(child, size))
            }
            _ => { 
                println!("{:?}", tokens);
                Err(TokenParserError)
            }

        }
    }
}


#[derive(Debug)]
/// An error thrown while attempting to parse an unexpected input string.
pub struct TokenParserError;

impl std::fmt::Display for TokenParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse string as filesystem api token")
    }
}

impl std::error::Error for TokenParserError {}