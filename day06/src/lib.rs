use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign, Index, IndexMut};

use futures::future::join_all;

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

    pub fn visited_directed(&self, direction: Direction) -> bool {
        if let SpaceState::Empty(dirs) = self {
            dirs[direction as usize]
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
                let mut d = dirs;
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

#[derive(Debug)]
pub enum MoveResult {
    Turn,
    Forward,
    Gone,
    Loop,
}

#[derive(Clone)]
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
                    if self[guy].visited_directed(Direction::Right) {
                        return MoveResult::Loop;
                    }
                    self.direction = Direction::Right;
                    self[guy] += Direction::Right;
                    return MoveResult::Turn;
                }
                let new_loc = (guy.0 - 1, guy.1);
                if self[new_loc].visited_directed(Direction::Up) {
                    return MoveResult::Loop;
                }
                self.location = new_loc;
                self[new_loc] += Direction::Up;
                MoveResult::Forward
            }
            Direction::Down => {
                if self.location.0 == Self::SIZE - 1 {
                    return MoveResult::Gone;
                }
                if self[(guy.0 + 1, guy.1)] == SpaceState::Obstacle {
                    if self[guy].visited_directed(Direction::Left) {
                        return MoveResult::Loop;
                    }
                    self.direction = Direction::Left;
                    self[guy] += Direction::Left;
                    return MoveResult::Turn;
                }
                let new_loc = (guy.0 + 1, guy.1);
                if self[new_loc].visited_directed(Direction::Down) {
                    return MoveResult::Loop;
                }
                self.location = new_loc;
                self[new_loc] += Direction::Down;
                MoveResult::Forward
            }
            Direction::Left => {
                if self.location.1 == 0 {
                    return MoveResult::Gone;
                }
                if self[(guy.0, guy.1 - 1)] == SpaceState::Obstacle {
                    if self[guy].visited_directed(Direction::Up) {
                        return MoveResult::Loop;
                    }
                    self.direction = Direction::Up;
                    self[guy] += Direction::Up;
                    return MoveResult::Turn;
                }
                let new_loc = (guy.0, guy.1 - 1);
                if self[new_loc].visited_directed(Direction::Left) {
                    return MoveResult::Loop;
                }
                self[new_loc] += Direction::Left;
                self.location = new_loc;
                MoveResult::Forward
            }
            Direction::Right => {
                if self.location.1 == Self::SIZE - 1 {
                    return MoveResult::Gone;
                }
                if self[(guy.0, guy.1 + 1)] == SpaceState::Obstacle {
                    if self[guy].visited_directed(Direction::Down) {
                        return MoveResult::Loop;
                    }
                    self.direction = Direction::Down;
                    self[guy] += Direction::Down;
                    return MoveResult::Turn;
                }
                let new_loc = (guy.0, guy.1 + 1);
                if self[new_loc].visited_directed(Direction::Right) {
                    return MoveResult::Loop;
                }
                self.location = new_loc;
                self[new_loc] += Direction::Right;
                MoveResult::Forward
            }
        }
    }

    pub async fn run_route(&mut self) -> MoveResult {
        loop {
            match self.move_guy() {
                MoveResult::Gone => return MoveResult::Gone,
                MoveResult::Loop => return MoveResult::Loop,
                _ => continue,
            }
        }
    }

    fn in_bounds(loc: &(usize, usize)) -> bool {
        loc.0 < Self::SIZE && loc.1 < Self::SIZE
    }

    pub async fn loop_obstacles(&mut self) -> HashSet<(usize, usize)> {
        let mut futures = vec![];
        loop {
            let (row, col) = self.location;
            let next = match self.direction {
                Direction::Up => (row - 1, col),
                Direction::Down => (row + 1, col),
                Direction::Left => (row, col - 1),
                Direction::Right => (row, col + 1),
            };

            if !Self::in_bounds(&next)
                || self[next].visited()
                || self.start == next
                || self[next] == SpaceState::Obstacle
            {
                match self.move_guy() {
                    MoveResult::Gone => break,
                    _ => continue,
                };
            }
            let mut potential = self.clone();
            potential[next] = SpaceState::Obstacle;

            futures.push(tokio::spawn(async move {
                let res = potential.run_route().await;
                match res {
                    MoveResult::Loop => Some(next),
                    _ => None,
                }
            }));

            match self.move_guy() {
                MoveResult::Gone => break,
                _ => continue,
            }
        }

        join_all(futures)
            .await
            .into_iter()
            .filter_map(|loc| loc.unwrap())
            .collect()
    }

    pub fn count_visited(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|state| state.visited())
            .count()
    }
}
