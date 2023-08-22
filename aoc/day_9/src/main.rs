use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
//// ANCHOR: states
/// Positions the Head of the rope can be in, relative to the Tail.
enum PositionH {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Center,
}
//// ANCHOR_END: states

#[derive(Clone, Copy, Debug)]
//// ANCHOR: transitions
/// Directions the Head of the rope can move, i.e. cardinal directions.
enum MoveH {
    Up,
    Right,
    Down,
    Left,
}
//// ANCHOR_END: transitions

impl TryFrom<char> for MoveH {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'U' => Ok(Self::Up),
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            _ => Err(InvalidCharError(value))
        }
    }
}

#[derive(Debug)]
pub struct InvalidCharError(char);
impl std::error::Error for InvalidCharError {}
impl std::fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot parse char {} as direction", self.0)
    }
}

//// ANCHOR: state_machine
pub struct StateMachine {
    /// Absolute position of tail
    abs_t: (isize, isize),      
    /// Relative position of head
    relative_h: PositionH,                
    /// Set of all previously visited locations
    pub visited: HashSet<(isize, isize)>,   
}
//// ANCHOR_END: state_machine

impl StateMachine {
//// ANCHOR: new
    pub fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0,0));
        
        Self {    
            abs_t: (0,0),
            relative_h: PositionH::Center,
            visited,
        }    
    }
//// ANCHOR_END: new
    fn parse_line(s: String) -> (MoveH, u32) {
        let mut t = s.split_whitespace();
        let dir = t.next()
        .unwrap()
        .chars()
        .next()
        .unwrap()
        .try_into()
        .unwrap();
        let rep = t.last().unwrap().parse().unwrap();
        (dir, rep)
    }

    fn step(&mut self, direction: MoveH, _num_steps: u32) {
        let (dx, dy) = match (self.relative_h, direction) {
            (PositionH::Up, MoveH::Up) => {
                (0, 1)
            }
            (PositionH::Up, MoveH::Left) => {
                self.relative_h = PositionH::UpLeft;
                (0,0)
            }
            (PositionH::Up, MoveH::Right) => {
                self.relative_h = PositionH::UpRight;
                (0,0)
            }
            (PositionH::UpRight, MoveH::Up) => {
                self.relative_h = PositionH::Up;
                (1,1)
            }
            (PositionH::UpRight, MoveH::Down) => {
                self.relative_h = PositionH::Right;
                (0,0)
            }
            (PositionH::UpRight, MoveH::Left) => {
                self.relative_h = PositionH::Up;
                (0,0)
            }
            (PositionH::UpRight, MoveH::Right) => {
                self.relative_h = PositionH::Right;
                (1,1)
            }
            (PositionH::Right, MoveH::Up) => {
                self.relative_h = PositionH::UpRight;
                (0,0)
            }
            (PositionH::Right, MoveH::Down) => {
                self.relative_h = PositionH::DownRight;
                (0,0)
            }
            (PositionH::Right, MoveH::Left) | (PositionH::Up, MoveH::Down) | (PositionH::Down, MoveH::Up) | (PositionH::Left, MoveH::Right) => {
                self.relative_h = PositionH::Center;
                (0,0)
            }
            (PositionH::Right, MoveH::Right) => {
                (1,0)
            }
            (PositionH::DownRight, MoveH::Up) => {
                self.relative_h = PositionH::Right;
                (0,0)
            }
            (PositionH::DownRight, MoveH::Down) => {
                self.relative_h = PositionH::Down;
                (1,-1)
            }
            (PositionH::DownRight, MoveH::Left) => {
                self.relative_h = PositionH::Down;
                (0,0)
            }
            (PositionH::DownRight, MoveH::Right) => {
                self.relative_h = PositionH::Right;
                (1,-1)
            }
            (PositionH::Down, MoveH::Down) => {
                (0, -1)
            }
            (PositionH::Down, MoveH::Left) => {
                self.relative_h = PositionH::DownLeft;
                (0,0)
            }
            (PositionH::Down, MoveH::Right) => {
                self.relative_h = PositionH::DownRight;
                (0,0)
            }
            (PositionH::DownLeft, MoveH::Up) => {
                self.relative_h = PositionH::Left;
                (0,0)
            }
            (PositionH::DownLeft, MoveH::Down) => {
                self.relative_h = PositionH::Down;
                (-1,-1)
            }
            (PositionH::DownLeft, MoveH::Left) => {
                self.relative_h = PositionH::Left;
                (-1,-1)
            }
            (PositionH::DownLeft, MoveH::Right) => {
                self.relative_h = PositionH::Down;
                (0,0)
            }
            (PositionH::Left, MoveH::Up) => {
                self.relative_h = PositionH::UpLeft;
                (0,0)
            }
            (PositionH::Left, MoveH::Down) => {
                self.relative_h = PositionH::DownLeft;
                (0,0)
            }
            (PositionH::Left, MoveH::Left) => {
                (-1,0)
            }
            (PositionH::UpLeft, MoveH::Up) => {
                self.relative_h = PositionH::Up;
                (-1,1)
            }
            (PositionH::UpLeft, MoveH::Down) => {
                self.relative_h = PositionH::Left;
                (0,0)
            }
            (PositionH::UpLeft, MoveH::Left) => {
                self.relative_h = PositionH::Left;
                (-1,1)
            }
            (PositionH::UpLeft, MoveH::Right) => {
                self.relative_h = PositionH::Up;
                (0,0)
            }
            (PositionH::Center, MoveH::Up) => {
                self.relative_h = PositionH::Up;
                (0,0)
            }
            (PositionH::Center, MoveH::Down) => {
                self.relative_h = PositionH::Down;
                (0,0)
            }
            (PositionH::Center, MoveH::Left) => {
                self.relative_h = PositionH::Left;
                (0,0)
            }
            (PositionH::Center, MoveH::Right) => {
                self.relative_h = PositionH::Right;
                (0,0)
            }
        };
        let (x, y) = self.abs_t;
        self.abs_t = (x + dx, y + dy);
        self.visited.insert(self.abs_t.clone());
    }

    pub fn execute_line(&mut self, s: String) {
        let (direction, num_steps) = Self::parse_line(s);
        for _ in 0..num_steps {
            self.step(direction, num_steps);
        }
    }
}

fn main() {
    let lines = aoc::read_as_lines("../inputs/day_9.txt").unwrap();

    let mut state = StateMachine::new();
    for line in lines {
        state.execute_line(line.unwrap());

    }
    println!("part 1: {:?}",state.visited.len());
}