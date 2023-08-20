pub struct Forest(Vec<Vec<Tree>>);

mod multi_thread;
mod single_thread;

impl Forest {
    pub fn new(path: &'static str) -> Self {
        let lines = aoc::read_as_lines(path).unwrap();

        let array = lines.map(|line| {

            let line = line.unwrap();
            
            line.chars().map(|c| {
                let height = c.to_digit(10).expect("Int parsing error");
                Tree::new(height as i32)
            })
            .collect::<Vec<Tree>>()

        })
        .collect::<Vec<Vec<Tree>>>();

        Self(array)
    }

    pub fn solve_single(&mut self) -> u32 {
        self.calc_visibility();
        self.sum_visibility()
    }

    pub fn solve_multi(&mut self) -> u32 {
        todo!();
    }
}

struct Tree {
    height: i32,
    visibility: bool,
}

impl Tree {
    fn new(height: i32) -> Self {
        Self {height, visibility: false}
    }
}