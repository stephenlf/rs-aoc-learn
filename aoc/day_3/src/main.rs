use std::collections::HashSet;

use aoc::*;

fn priority(c: &char) -> u32 {
    let val = *c as u32;
    match val {
        65..=90 => val-65+27,
        97..=122 => val-96,
        _ => 0,
    }
}

fn intersection(line: String) -> char {
    let mut left_side: HashSet<char> = HashSet::new();
    let mut right_side: HashSet<char> = HashSet::new();
    let middle = line.len() / 2;

    for (i, c) in line.chars().enumerate() {
        if i < middle {
            left_side.insert(c);
        }
        else {
            right_side.insert(c);
        }
    }

    let mut intersection_iter = left_side.intersection(&right_side);
    
    let intersection = intersection_iter.next().unwrap().to_owned();
    assert!(intersection_iter.next().is_none());    // Assert there is only one element in intersection.
    
    intersection    
}

fn main() {
    let lines = read_as_lines("../inputs/day_3.txt").unwrap();

    let total = lines.map(|line| {
        let c = intersection(line.unwrap());
        priority(&c)
    }).sum::<u32>();

    println!("Part 1: {total}");
}
