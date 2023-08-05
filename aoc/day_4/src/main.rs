/// Stores start and end of input range, inclusive
#[derive(PartialEq, Eq, Debug)]
struct Range(u32, u32);

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

impl TryFrom<&str> for Range {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {        
        let range = value.split_once('-');
        let (min, max) = if range.is_some() {
            range.unwrap()
        } else {
            return Err("Could not find '-' token".into());
        };

        let min = min.parse::<u32>()?;    
        let max = max.parse::<u32>()?;

        Ok(Self(min, max))
    }
}

fn ranges_from_line(line: String) -> Result<(Range, Range), Box<dyn std::error::Error>> {
    let ranges = line.split_once(',');
    let (left, right) = if ranges.is_some() {
        ranges.unwrap()
    } else {
        return Err("Could not find ',' token".into());
    };

    let left = Range::try_from(left)?;
    let right = Range::try_from(right)?;

    Ok((left, right))
}



fn main() {
    let lines = aoc::read_as_lines("../inputs/day_4.txt").unwrap();
    let total = lines.fold(0_u32, |mut accum, line| {
        let (left_range, right_range) = ranges_from_line(line.unwrap()).unwrap();
        if left_range.contains(&right_range) || right_range.contains(&left_range) {
            accum += 1;
        }
        accum
    });
    println!("Part 1: {total}");

}

#[cfg(test)]
mod day_4 {
    use super::*;

    #[test]
    fn test_range_from_string() {
        let test_string = "1-2";
        let test_range = Range::try_from(test_string).unwrap();
        assert_eq!(test_range, Range(1, 2));
        
        // We should be prepared for multi-digit numbers
        let test_string = "10-20";
        let test_range = Range::try_from(test_string).unwrap();
        assert_eq!(test_range, Range(10, 20));
    }
}