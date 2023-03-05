// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x,
            y,
            dir: d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.dir = match self.dir {
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
        };

        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.dir = match self.dir {
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
        };

        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.dir {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        };

        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.to_uppercase().chars() {
            if c == 'R' {
                self = self.turn_right();
            }

            if c == 'L' {
                self = self.turn_left();
            }

            if c == 'A' {
                self = self.advance();
            }

            // match c {
            //     'R' => self.turn_right(),
            //     'L' => self.turn_left(),
            //     'A' => self.advance(),
            //     _ => &mut self,
            // };
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
