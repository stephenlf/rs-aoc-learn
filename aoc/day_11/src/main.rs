use aoc;
use day_11::*;
use std::{rc::Rc, cell::RefCell};


fn main() {
    // Initialize monkeys
    let mut lines = aoc::read_as_lines("../inputs/day_11.txt").unwrap();
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];
    while let Some(monkey) = Monkey::new(&mut lines) {
        monkeys.push(Rc::new(RefCell::new(monkey)));
    }

    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let items = monkey.borrow_mut().throw_items();
            for (monkey_id, item) in items {
                monkeys[monkey_id].borrow_mut().items.push_back(item);
            }
        }
    }

    for monkey in monkeys {
        println!("{:?}", monkey);
    }

}

