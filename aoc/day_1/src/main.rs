use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

fn read_as_lines<T: ToString>(path: T) -> Lines<BufReader<File>> {
    let file = File::open(PathBuf::from(path.to_string())).expect("Could not open file");
    BufReader::new(file).lines()
}

#[derive(Default)]
struct Expedition(Vec<Elf>);

impl Expedition {
    fn new() -> Self {
        Default::default()
    }

    fn add_elf(&mut self, elf: Elf) {
        self.0.push(elf);
        // Can be done in 1 step with `self.0.push(Elf {calories} );`
    }

    fn max(&self) -> u32 {
        self.0.iter()
            .map(|elf| elf.total_calories())    // Replaces each Elf with result of elf.total_calories() in place.
            .max().unwrap()
    }
}

#[derive(Default)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn new() -> Self {
        Self { calories: vec![] }
    }

    fn add_calorie(&mut self, calorie: u32) {
        self.calories.push(calorie);
    }

    fn total_calories(&self) -> u32 {
        self.calories.iter().sum::<u32>()
    }
}

fn expedition_builder(mut lines: Lines<BufReader<File>>) -> Expedition {
    let mut expedition = Expedition::new();
    expedition.add_elf(Elf::new());

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if &line[..] == "\n" || line.is_empty() {
            expedition.add_elf(Elf::new());
        } else {
            expedition.0.last_mut().unwrap()                // Get most recent elf
                .add_calorie(line.parse::<u32>().unwrap())          // Parse line as u32 and add to elf
        }
    }
    expedition
}

fn main() {
    // PART 1
    // Input file path is relative to package directory: aoc/day_1
    let lines = read_as_lines("../inputs/day_1.txt");
    let expedition = expedition_builder(lines);
    let max_cal = expedition.max();
    println!("{}", max_cal);

    // PART 2
    let mut expedition_calories = expedition.0.iter()       // Create iterator over elves in expedition
        .map(|elf| elf.total_calories())        // Replace each elf in the iterator with the sum of its calories
        .collect::<Vec<u32>>();                                     // Convert iterator back into a vector in order to use Vec::sort()

    expedition_calories.sort();
    expedition_calories.reverse();                  // Not the most efficient way to do this.

    let top_three_sum = expedition_calories[..3].iter().sum::<u32>();       // Convert back into iterator to use Iter::sum::<T>()
    println!("{}", top_three_sum);
}

#[cfg(test)]
mod day_1 {
    use super::*;

    #[test]
    fn test_new_elf() {
        let elf = Elf::new();
        assert_eq!(elf.calories.len(), 0);
    }

    #[test]
    fn test_add_calorie() {
        let mut elf = Elf::new();
        elf.add_calorie(1);
        elf.add_calorie(2);
        elf.add_calorie(3);
        assert_eq!(elf.calories, vec![1, 2, 3]);
    }

    #[test]
    fn test_total_calorie() {
        let mut elf = Elf::new();
        elf.add_calorie(1);
        elf.add_calorie(2);
        elf.add_calorie(3);
        assert_eq!(elf.total_calories(), 6);
    }

    #[test]
    fn test_add_elf_to_expedition() {
        let mut expedition = Expedition::new();
        expedition.add_elf(Elf {calories: vec![1,2,3] });
        expedition.add_elf(Elf {calories: vec![10,20,30] });
        expedition.add_elf(Elf {calories: vec![100,200,300] });
        assert_eq!(expedition.max(), 600);
    }
}