use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign, Sub, SubAssign};

type LocationList = Vec<Vector>;
type Map = HashMap<char, LocationList>;
const SIZE: isize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(isize, isize);

impl Vector {
    fn in_bounds(&self) -> bool {
        !self.0.is_negative() && self.0 < SIZE && !self.1.is_negative() && self.1 < SIZE
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Vector> for Vector {

    fn sub_assign(&mut self, rhs: Vector) {
        
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}


impl AddAssign<Vector> for Vector {

    fn add_assign(&mut self, rhs: Vector) {
        
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}


pub fn read_input<P>(path: P) -> Map
where
    P: AsRef<std::path::Path>,
{
    let lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    let mut out = HashMap::new();

    for (i, line) in lines.enumerate() {
        for (ii, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                freq => {
                    out.entry(freq)
                        .or_insert_with(Vec::new)
                        .push(Vector(i as isize, ii as isize));
                }
            }
        }
    }
    out
}

pub fn find_nodes(map: &Map) -> LocationList {
    let mut out = HashSet::new();

    for set in map.values() {
        for (i, &tower) in set.iter().enumerate() {
            for &other in set.iter().skip(i + 1) {
                let diff = tower - other;
                let node = other - diff;
                if node.in_bounds() {
                    out.insert(node);
                }
                let node = tower + diff;
                if node.in_bounds() {
                    out.insert(node);
                }
            }
        }
    }

    out.into_iter().collect()
}

pub fn find_nodes_harmonic(map: &Map) -> LocationList {
    let mut out = HashSet::new();

    for set in map.values() {
        for (i, &tower) in set.iter().enumerate() {
            for &other in set.iter().skip(i + 1) {
                let diff = tower - other;
                let mut node = other;
                while node.in_bounds() {
                    out.insert(node);
                    node -= diff;
                }
                let mut node = tower;
                while node.in_bounds() {
                    out.insert(node);
                    node += diff;
                }
            }
        }
    }

    out.into_iter().collect()

}
