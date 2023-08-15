use std::collections::HashSet;

fn find_signal_start(signal: String) -> usize {
    let signal: Vec<char> = signal.chars().collect();
    
    for (i, window) in signal.as_slice()
        .windows(4)
        .enumerate() 
    {
        if is_unique(window) {
            return i + 4;
        }
    }
    panic!("Could not find a valid signal start sequence");
}

fn is_unique(window: &[char]) -> bool {
    assert_eq!(window.len(), 4);

    // Throw each character into a set. 
    let mut set: HashSet<char> = HashSet::with_capacity(4);
    for c in window {
        set.insert(*c);
    }

    // Sets only keep unique entries, so a set length of four means that all 
    // entries were unique. Depends on the assertion that window.len() == 4.
    if set.len() == 4 {
        true
    } else {
        false
    }
}

fn main() {
    let input_path = std::path::PathBuf::from("../inputs/day_6.txt");
    let input = std::fs::read_to_string(input_path).unwrap();
    println!("Day 1: {}", find_signal_start(input));
}

#[cfg(test)] 
mod day_6 {
    use super::*;

    #[test]
    fn test_windows() {
        let test_1 = String::from("abcdxxxxyyyyzzzz");
        assert_eq!(find_signal_start(test_1), 4);

        let test_2 = String::from("aaaabbbbccccxyz");
        assert_eq!(find_signal_start(test_2), 15);

        let test_3 = String::from("axayza");
        assert_eq!(find_signal_start(test_3), 5);
    }
}