use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

/// Takes in the path to a file, and returns an iterator over the lines of that
/// file. File paths may be absolute or relative to the package, and should be
/// forward-slash delimited (even on Windows systems.) 
pub fn read_as_lines<T: ToString>(path: T) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(PathBuf::from(path.to_string()))?;

    Ok(BufReader::new(file).lines())
}