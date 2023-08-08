use aoc;
use std::iter::Peekable;

mod port;

fn parse_command(line: &String) -> (u32, usize, usize) {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();
    let num_crates = tokens[1].parse::<u32>()
        .expect("Could not parse num_crates");
    let origin = tokens[3].parse::<usize>().unwrap();
    let dest = tokens[5].parse::<usize>().unwrap();

    (num_crates, origin, dest)
}

fn main() {
    let mut lines: Peekable<aoc::LinesIter> = 
        aoc::read_as_lines("../inputs/day_5.txt").unwrap().peekable();

    let mut port = port::Port::new(&mut lines);

    let _ =lines.next();        // Consumes spacer line between port digram and command list

    for line in lines {
        let (num_crates , origin , dest) = parse_command(&line.unwrap());
        port.arrange(num_crates, origin, dest);
    }

    port.print();

}