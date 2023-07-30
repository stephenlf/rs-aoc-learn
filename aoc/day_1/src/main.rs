struct Expedition(Vec<Elf>);

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
}

fn main() {}

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
}