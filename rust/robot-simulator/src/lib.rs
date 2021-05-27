// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Robot {
            position: self.position,
            direction,
        }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        Robot {
            position: self.position,
            direction,
        }
    }

    pub fn advance(self) -> Self {
        let (mut x, mut y) = self.position;
        match self.direction {
            Direction::East => x += 1,
            Direction::West => x -= 1,
            Direction::North => y += 1,
            Direction::South => y -= 1,
        }

        Robot {
            position: (x, y),
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            robot = match c {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => continue,
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
