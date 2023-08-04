use aoc::read_as_lines;

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

impl TryFrom<char> for Hand {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Self::Rock(1)),
            'B' | 'Y' => Ok(Self::Paper(2)),
            'C' | 'Z' => Ok(Self::Scissors(3)),
            _ => Err("Unexpected character"),
        }
    }
}

fn game(my_hand: &Hand, opp_hand: &Hand) -> u32 {
    use Hand::*;            // Lets us write `Rock` instead of `Hand::Rock`, etc.
    match (my_hand, opp_hand) {
        (Rock(n), Scissors(_)) | (Paper(n), Rock(_))     | (Scissors(n), Paper(_))    => 6 + n,  // win
        (Rock(n), Rock(_))     | (Paper(n), Paper(_))    | (Scissors(n), Scissors(_)) => 3 + n,  // tie
        (Rock(n), Paper(_))    | (Paper(n), Scissors(_)) | (Scissors(n), Rock(_))     => 0 + n,  // loss
    }
}

fn main() {
    let lines = read_as_lines("../inputs/day_2.txt").unwrap();

    let mut total = 0;
    
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();
        let opp_hand = Hand::try_from(chars.next().unwrap()).unwrap();
        let my_hand = Hand::try_from(chars.last().unwrap()).unwrap();
        total += game(&my_hand, &opp_hand);
    }

    println!("Part 1: {total}");
}

#[cfg(test)]
mod day_2 {
    use std::f32::consts::E;

    use super::*;

    #[test]
    fn test_game() {
        let win = (Hand::Rock(1), Hand::Scissors(0));       // (My hand, opponent hand)
        let tie = (Hand::Paper(2), Hand::Paper(0));
        let loss = (Hand::Scissors(3), Hand::Rock(0));
        assert_eq!(game(&win.0, &win.1), 6 + 1);     // win score + hand score
        assert_eq!(game(&tie.0, &tie.1), 3 + 2);
        assert_eq!(game(&loss.0,&loss.1), 0 + 3);
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Hand::try_from('A'), Ok(Hand::Rock(1)));
        assert_eq!(Hand::try_from('B'), Ok(Hand::Paper(2)));
        assert_eq!(Hand::try_from('C'), Ok(Hand::Scissors(3)));
        assert!(Hand::try_from('h').is_err());
        // etc...
    }

    #[test]
    fn test_hs() {
        use std::collections::HashSet;

        let hs1 = HashSet::from([1,2,3]);
        let hs2 = HashSet::from([2,3,4]);

            // Iterate over first hashset
        let mut intersection = hs1.iter()       
            // Check if each element is in second hashset
            .filter(move |element| hs2.contains(element));   

        let element = intersection.next().unwrap();
        assert!(*element == 2 || *element == 3);

        let element = intersection.next().unwrap();
        assert!(*element == 2 || *element == 3);

        assert!(intersection.next().is_none());
    }
}