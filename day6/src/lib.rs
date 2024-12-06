use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceState {
    Obstacle,
    Empty([bool; 4]),
}

impl Default for SpaceState {
    fn default() -> Self {
        Self::Empty([false, false, false, false])
    }
}

impl SpaceState {
    pub fn visited(&self) -> bool {
        if let SpaceState::Empty([up, down, left, right]) = self {
            *up || *down || *left || *right
        } else {
            false
        }
    }
}

impl Add<Direction> for SpaceState {
    type Output = SpaceState;

    fn add(self, rhs: Direction) -> Self::Output {
        Self::Empty(match self {
            Self::Empty(dirs) => {
                let mut d = dirs.clone();
                d[rhs as usize] = true;
                d
            }
            _ => {
                let mut dirs = [false, false, false, false];
                dirs[rhs as usize] = true;
                dirs
            }
        })
    }
}

impl AddAssign<Direction> for SpaceState {
    fn add_assign(&mut self, rhs: Direction) {
        match self {
            Self::Empty(dirs) => {
                dirs[rhs as usize] = true;
            }
            _ => {
                let mut dirs = [false, false, false, false];
                dirs[rhs as usize] = true;
                *self = Self::Empty(dirs);
            }
        };
    }
}

pub enum MoveResult {
    Turn((usize, usize)),
    Forward((usize, usize)),
    Gone,
}

pub struct Map {
    map: Vec<Vec<SpaceState>>,
    start: (usize, usize),
    location: (usize, usize),
    direction: Direction,
}

impl Index<(usize, usize)> for Map {
    type Output = SpaceState;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.map[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.map[index.0][index.1]
    }
}

impl Map {
    const SIZE: usize = 130;

    pub fn from_file<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let map = [[SpaceState::default()]
            .into_iter()
            .cycle()
            .take(130)
            .collect()]
        .into_iter()
        .cycle()
        .take(130)
        .collect();
        let mut out = Self {
            location: (0, 0),
            direction: Direction::Up,
            map,
            start: (0, 0),
        };
        let lines = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|line| line.unwrap());
        for (i, row) in lines.enumerate() {
            for (ii, char) in row.chars().enumerate() {
                match char {
                    '^' => {
                        out.location = (i, ii);
                        out.start = (i, ii);
                        out.direction = Direction::Up;
                        out[(i, ii)] += Direction::Up;
                    }
                    '#' => out[(i, ii)] = SpaceState::Obstacle,
                    _ => (),
                }
            }
        }
        out
    }

    fn move_guy(&mut self) -> MoveResult {
        let guy = self.location;
        match self.direction {
            Direction::Up => {
                if self.location.0 == 0 {
                    return MoveResult::Gone;
                }
                if self[(guy.0 - 1, guy.1)] == SpaceState::Obstacle {
                    self.direction = Direction::Right;
                    self[guy] += Direction::Right;
                    return MoveResult::Turn(guy);
                }
                let new_loc = (guy.0 - 1, guy.1);
                self.location = new_loc;
                self[new_loc] += Direction::Up;
                MoveResult::Forward(self.location)
            }
            Direction::Down => {
                if self.location.0 == Self::SIZE - 1 {
                    return MoveResult::Gone;
                }
                if self[(guy.0 + 1, guy.1)] == SpaceState::Obstacle {
                    self.direction = Direction::Left;
                    self[guy] += Direction::Left;
                    return MoveResult::Turn(guy);
                }
                let new_loc = (guy.0 + 1, guy.1);
                self.location = new_loc;
                self[new_loc] += Direction::Down;
                MoveResult::Forward(self.location)
            }
            Direction::Left => {
                if self.location.1 == 0 {
                    return MoveResult::Gone;
                }
                if self[(guy.0, guy.1 - 1)] == SpaceState::Obstacle {
                    self.direction = Direction::Up;
                    self[guy] += Direction::Up;
                    return MoveResult::Turn(guy);
                }
                let new_loc = (guy.0, guy.1 - 1);
                self[new_loc] += Direction::Left;
                self.location = new_loc;
                MoveResult::Forward(self.location)
            }
            Direction::Right => {
                if self.location.1 == Self::SIZE - 1 {
                    return MoveResult::Gone;
                }
                if self[(guy.0, guy.1 + 1)] == SpaceState::Obstacle {
                    self.direction = Direction::Down;
                    self[guy] += Direction::Down;
                    return MoveResult::Turn(guy);
                }
                let new_loc = (guy.0, guy.1 + 1);
                self.location = new_loc;
                self[new_loc] += Direction::Right;
                MoveResult::Forward(self.location)
            }
        }
    }

    pub fn run_route(&mut self) {
        loop {
            if let MoveResult::Gone = self.move_guy() {
                break;
            }
        }
    }

    pub fn count_visited(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|state| state.visited())
            .count()
    }
}
