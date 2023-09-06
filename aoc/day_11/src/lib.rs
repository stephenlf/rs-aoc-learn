use std::collections::VecDeque;
use aoc;
use std::fmt::Debug;

pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<usize>,
    pub operation: Box<dyn Fn(usize) -> usize>,  // NEW
    pub test: Box<dyn Fn(usize) -> bool>,        // NEW
    pub target_if_true: usize,
    pub target_if_false: usize,
    pub touch_counter: usize,
}

impl Monkey {
    /// Initialize a new monkey from a Lines iterator.
    pub fn new(lines: &mut aoc::LinesIter) -> Option<Self> {
        // Check if first line is "None". If so, return early. Otherwise, continue parsing.
        let first_line: String = if let Some(line) = lines.next() {
            line.unwrap()
        } else {
            return None
        };

        // Pass each line into relevant parser. Note that the first line was already
        // read, so we pass in that variable instead of calling `lines.next()` again.
        let id = Self::parse_id(first_line);
        let items = Self::parse_items(lines.next().unwrap().unwrap());
        let operation = Self::parse_operation(lines.next().unwrap().unwrap());
        let test = Self::parse_test(lines.next().unwrap().unwrap());
        let target_if_true = Self::parse_true_monkey(lines.next().unwrap().unwrap());
        let target_if_false = Self::parse_false_monkey(lines.next().unwrap().unwrap());
        let _ = lines.next();

        Some( Self {
            id,
            items,
            operation,
            test,
            target_if_true,
            target_if_false,
            touch_counter: 0,
        })
    }

    /// A single turn. The output is given as a list of pairs of numbers 
    /// Vec<(usize, usize)> where item.0 is the target monkey and item.1
    /// is the item to add to the stack.
    pub fn throw_items(&mut self) -> Vec<(usize, usize)> {
        let mut thrown_items: Vec<(usize, usize)> = vec![];

        while let Some(item) = self.items.pop_front() {
            self.touch_counter += 1;
            let item = (self.operation)(item) / 3;
            match (self.test)(item) {
                true => thrown_items.push((self.target_if_true, item)),
                false => thrown_items.push((self.target_if_false, item)),

            }
        }
        
        thrown_items
    }
    
    /// Pulls monkey.id from the first line of each monkey block the input
    fn parse_id(line: String) -> usize { 
        // Example input: "Monkey 0:"
        assert_eq!(&line[..7], "Monkey ");
        let line = line.strip_suffix(':').unwrap();
        (&line[7..]).parse().unwrap()
    }

    /// Creates a VecDeque populated with items in the second line of the block
    fn parse_items(line: String) -> VecDeque<usize> { 
        // Example input: "  Starting items: 66, 59, 64, 51"
        assert_eq!(&line[..18], "  Starting items: ");
        
        let item_iter = (&line[18..])
            .split(',')
            .map(|item| item.trim().parse::<usize>().unwrap());

        VecDeque::from_iter(item_iter)
    }

    /// Creates a closure matching `operation`, the third line of the block
    fn parse_operation(line: String) -> Box<dyn Fn(usize) -> usize> { 
        // Example input: "  Operation: new = old + 2"
        let line = line.trim();
        assert_eq!(&line[..21], "Operation: new = old ");

        let tokens = (&line[21..]).split_whitespace().collect::<Vec<&str>>();
        match (tokens[0], tokens[1]) {
            ("+", "old") => {
                let closure = move |x: usize| x + x;
                Box::new(closure)
            },
            ("*", "old") => {
                let closure = move |x: usize| x * x;
                Box::new(closure)
            },
            ("+", n) => {
                let operand = n.parse::<usize>().unwrap();
                let closure = move |x: usize| x + operand;
                Box::new(closure)
            },
            ("*", n) => {
                let operand = n.parse::<usize>().unwrap();
                let closure = move |x: usize| x * operand;
                Box::new(closure)
            },
            _ => panic!("Unexpect token in line {}", line)
        }
    }

    /// Creates a closure matching `test`, the fourth line of the block
    fn parse_test(line: String) -> Box<dyn Fn(usize) -> bool> { 
        // Example input: "  Test: divisible by 11"
        assert_eq!(&line[..21], "  Test: divisible by ");
        let divisor = (&line[21..]).parse::<usize>().unwrap();

        let closure = { move |x: usize| x % divisor == 0 };

        Box::new(closure)
    }

    /// Pulls the id of the target monkey (if test passes) from the fifth line
    fn parse_true_monkey(line: String) -> usize { 
        // Example input: "    If true: throw to monkey 1"
        assert_eq!(&line[..29], "    If true: throw to monkey ");
        (&line[29..]).parse().unwrap()
    }
    
    /// Pulls the id of the target monkey (if test fails) from the fifth line
    fn parse_false_monkey(line: String) -> usize {
        // Example input: "    If false: throw to monkey 4"
        assert_eq!(&line[..30], "    If false: throw to monkey ");
        (&line[30..]).parse().unwrap()
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {} with items {:?}", self.id, self.items)
    }
}

#[cfg(test)]
mod day_11 {
    #[test]
    fn parse_true_false() {
        let input = String::from("    If true: throw to monkey 1");
        let n = super::Monkey::parse_true_monkey(input);
        assert_eq!(n, 1);

        let input = String::from("    If false: throw to monkey 12");
        let n = super::Monkey::parse_false_monkey(input);
        assert_eq!(n, 12);
    }

    #[test]
    fn parse_id() {
        let input = String::from("Monkey 12:");
        let id = super::Monkey::parse_id(input);
        assert_eq!(id, 12);
    }

    #[test]
    fn parse_test() {
        let input = String::from("  Test: divisible by 11");
        let divisible_by_11 = super::Monkey::parse_test(input);

        assert_eq!(divisible_by_11(22), true);
        assert_eq!(divisible_by_11(23), false);
    }

    #[test]
    fn parse_operation() {
        let input = String::from("  Operation: new = old * 19");
        let operation = super::Monkey::parse_operation(input);
        assert_eq!(operation(3), 3 * 19);

        let input = String::from("  Operation: new = old + 19");
        let operation = super::Monkey::parse_operation(input);
        assert_eq!(operation(3), 3 + 19);

        let input = String::from("  Operation: new = old * old");
        let operation = super::Monkey::parse_operation(input);
        assert_eq!(operation(3), 3 * 3);

        let input = String::from("  Operation: new = old + old");
        let operation = super::Monkey::parse_operation(input);
        assert_eq!(operation(3), 3 + 3);
    }
}