use aoc::LinesIter;
use core::num;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Port(Vec<Dock>);

impl Port {
    /// Creates a new port from ...some input???
    pub fn new(lines: &mut Peekable<LinesIter>) -> Self {
        let mut port = Self(Vec::new());
        let line: &String = lines.peek()    // Option<&Result<String>>
            .unwrap()       // &Result<String>
            .as_ref()       // Result<&String>
            .unwrap();      // &String

        port.create_docks(line);        // Empty docks instantiated

        loop {
            let line = lines.next() // Option<Result<String>>
                .unwrap().unwrap();

            // Check if first character is whitespace, indicating 
            // end of port diagram
            if line.chars().nth(0).unwrap().is_whitespace() {
                break;
            }

            port.populate_dock_from_line(&line);
        }

        // Docks were instantiated backwards, so we have to reverse them
        for dock in &mut port.0 {
            dock.0.reverse();
        }

        port
    }

    /// Moves `num_crates` number of crates from the top of stack `origin`
    /// to the top of stack `destination`. Crates are moved one at a time.
    /// Origin and dest indices are offset by one to match input.
    pub fn arrange(&mut self, num_crates: u32, origin: usize, dest: usize) {
        for _ in 0..num_crates {
            let c = self.0[origin - 1].0.pop().unwrap();
            self.0[dest - 1].0.push(c);
        }
    }

    /// Prints top crate of each Dock to stout.
    pub fn print(&self) {
        println!("{}", todo!());
    }

    fn create_docks(&mut self, line: &String) {
        for _ in 0..Self::num_docks(line) {
            self.0.push(Default::default());
        }
    }

    fn num_docks(line: &String) -> usize {
        (line.len() + 1) / 4
    }

    fn populate_dock_from_line(&mut self, line: &String) {
        let mut chars = line.chars();

        // Get first character and push to first dock
        let mut c = chars.nth(1).unwrap();
        if c.is_alphabetic() {
            self.0[0].0.push(c);
        }

        let num_docks = self.0.len();

        // Grab nth character and pus
        for i in 1..num_docks {
            c = chars.nth(3).unwrap();
            if c.is_alphabetic() {
                self.0[i].0.push(c)
            }
        }
    }
}

#[derive(Default, Debug)]
struct Dock(Vec<char>);

#[cfg(test)]
mod day_5 {
    use super::*;

    fn init_lines() -> Peekable<aoc::LinesIter> {
        aoc::read_as_lines("../inputs/day_5.txt").unwrap().peekable()
    }

    #[test]
    fn new() {
        let port = Port::new(&mut init_lines());
        assert_eq!(port.0[0].0[0], 'H');
        assert_eq!(*port.0[0].0.last().unwrap(), 'S');

        assert_eq!(port.0.last().unwrap().0[0], 'M');
        assert_eq!(*port.0.last().unwrap().0.last().unwrap(), 'T');
    }

    #[test]
    fn arrange() {
        let mut port = Port(vec![
            Dock(vec!['A', 'B', 'C']),
            Dock(vec!['D', 'E', 'F']),
        ]);
        port.arrange(2, 1, 2);
        assert_eq!(port.0[0].0, vec!['A']);
        assert_eq!(port.0[1].0, vec!['D', 'E', 'F', 'C', 'B']);
    }
}