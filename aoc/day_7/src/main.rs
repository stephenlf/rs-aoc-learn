use aoc::*;

mod filesystem;
use filesystem::*;

mod token;
use token::Token;


fn main() {
    let lines = read_as_lines("../inputs/day_7.txt").unwrap();

    let mut fs = Filesystem::new();

    for line in lines {
        let token = Token::try_from(line.unwrap()).unwrap();
        match token {
            Token::CdRoot => fs.cd_root(),
            Token::CdParent => fs.cd_parent(),
            Token::CdChild(child) => fs.cd_child(&child),
            Token::Ls => continue,
            Token::Dir(dir_name) => fs.ls_folder(&dir_name),
            Token::File(file_name, file_size) => fs.ls_file(file_name, file_size),
        }
    }

    fs.update_all();
    println!("Filesystem: {}", fs);
}