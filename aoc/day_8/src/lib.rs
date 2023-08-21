use std::{
    sync::{Arc, mpsc},
    thread
};

pub struct Forest(Vec<Vec<Tree>>);

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

    #[inline]
    pub fn calc_visibility(&mut self) {
        let (rows, columns) = (self.0.len(), self.0[0].len());
        let mut max_height: i32;

        // Left to right
        for i in 0..rows {
            max_height = -1;
            for j in 0..columns {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }

        // Top to bottom
        for j in 0..columns {
            max_height = -1;
            for i in 0..rows {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }

        // right to left
        for i in 0..rows {
            max_height = -1;
            for j in (0..columns).rev() {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }

        // bottom to top
        for j in 0..columns {
            max_height = -1;
            for i in (0..rows).rev() {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visibility = true;
                }
            }
        }
    }

    pub fn sum_visibility(&self) -> u32 {
        self.0.iter().flatten().fold(0, |accum, tree| {
            if tree.visibility {
                accum + 1
            } else {
                accum
            }
        })
    }

    #[inline]
    pub fn calc_multi(&mut self) {
        let heights: Vec<Vec<i32>> = self.0.iter().map(|row| {
            row.iter().map(|tree| tree.height).collect()
        }).collect();

        let heights = Arc::new(heights);

        // Threads will send messages of the form (row, column, visibility)
        let (tx, rx) = mpsc::channel::<(usize, usize)>();
        
        let heights_clone = heights.clone();
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || 
            Self::scan_from(Direction::Bottom, heights_clone, tx_clone)
        );
        
        let heights_clone = heights.clone();
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || 
            Self::scan_from(Direction::Top, heights_clone, tx_clone)
        );
        
        let heights_clone = heights.clone();
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || 
            Self::scan_from(Direction::Left, heights_clone, tx_clone)
        );
        
        let heights_clone = heights.clone();
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || 
            Self::scan_from(Direction::Right, heights_clone, tx_clone)
        );

        // Required for loop to finish
        drop(tx);

        for (i, j) in rx {
            self.0[i][j].visibility = true;
        }
    }

    #[inline]
    fn scan_from(direction: Direction, tree_grid: Arc<Vec<Vec<i32>>>, tx: mpsc::Sender<(usize, usize)>) {
        let (rows, columns) = (tree_grid.len(), tree_grid[0].len());
        let mut max_height: i32;

        match direction {
            Direction::Left => {
                for i in 0..rows {
                    max_height = -1;
                    for j in 0..columns {
                        let tree_height = tree_grid[i][j];
                        if tree_height > max_height {
                            max_height = tree_height;
                            tx.send((i, j)).unwrap();
                        }
                    }
                }
            },
            Direction::Top => {
                for j in 0..columns {
                    max_height = -1;
                    for i in 0..rows {
                        let tree_height = tree_grid[i][j];
                        if tree_height > max_height {
                            max_height = tree_height;
                            tx.send((i, j)).unwrap();
                        }
                    }
                }
            },
            Direction::Right => {
                for i in 0..rows {
                    max_height = -1;
                    for j in (0..columns).rev() {
                        let tree_height = tree_grid[i][j];
                        if tree_height > max_height {
                            max_height = tree_height;
                            tx.send((i, j)).unwrap();
                        }
                    }
                }
            },
            Direction::Bottom => {
                for j in 0..columns {
                    max_height = -1;
                    for i in (0..rows).rev() {
                        let tree_height = tree_grid[i][j];
                        if tree_height > max_height {
                            max_height = tree_height;
                            tx.send((i, j)).unwrap();
                        }
                    }
                }
            },
        }        
    }
}

enum Direction {
    Top,
    Bottom,
    Left,
    Right
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