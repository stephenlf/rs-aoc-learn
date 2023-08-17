struct Forest(Vec<Vec<Tree>>);

impl Forest {
    fn new(path: &'static str) {
        let lines = aoc::read_as_lines(path).unwrap();

        let array = lines.map(|line| {

            let line = line.unwrap();
            
            line.chars().map(|c| {
                let height = c.to_digit(10).expect("Int parsing error");
                Tree::new(height)
            })
            .collect::<Vec<Tree>>()

        })
        .collect::<Vec<Vec<Tree>>>();
    }
}

struct Tree {
    height: u32,
    visibility: bool,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {height, visibility: false}
    }
}