use aoc;
use day_11::*;
use std::cell::RefCell;


fn main() {
    // Initialize monkeys
    let mut lines = aoc::read_as_lines("../inputs/day_11.txt").unwrap();

    // In our loop of rounds, we will need to pull from one monkey and push to 
    // another, all without breaking the loop. The borrow checker won't let us
    // do that with monkeys stored in Vec<Monkey>, so we wrap them in RefCell
    // to allow for run-time borrow checks.
    let mut monkeys: Vec<RefCell<Monkey>> = vec![];
    while let Some(monkey) = Monkey::new(&mut lines) {
        monkeys.push(RefCell::new(monkey));
    }

    // Simulate rounds 1-20
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let items = monkey.borrow_mut().throw_items();
            for (monkey_id, item) in items {
                monkeys[monkey_id].borrow_mut().items.push_back(item);
            }
        }
    }

    let mut touches = monkeys.iter()
        .map(|monkey| monkey.borrow().touch_counter)
        .collect::<Vec<_>>();

    // Sorts in reverse order, largest to smallest.
    touches.sort_by(|a, b|b.cmp(a));

    println!("{:?}", touches);

    println!("Part 1: {}", touches[0] * touches[1]);

}

