use std::collections::VecDeque;

pub struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,  // NEW
    test: Box<dyn Fn(usize) -> bool>,        // NEW
    target_if_true: usize,
    target_if_false: usize,
}

impl Monkey {
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
    fn parse_operation(line: String) -> Box<dyn Fn(usize) -> usize> { todo!() }

    /// Creates a closure matching `test`, the fourth line of the block
    fn parse_test(line: String) -> Box<dyn Fn(usize) -> bool> { todo!() }

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
}