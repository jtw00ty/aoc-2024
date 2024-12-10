use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;

pub struct Map(Vec<Vec<u8>>);

impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl Map {
    const SIZE: usize = 59;

    fn in_bounds(loc: &(usize, usize)) -> bool {
        loc.0 < Self::SIZE && loc.1 < Self::SIZE
    }

    fn trailheads(&self) -> Vec<(usize, usize)> {
        let mut out = vec![];
        for i in 0..Self::SIZE {
            for ii in 0..Self::SIZE {
                if self[(i, ii)] == 0 {
                    out.push((i, ii));
                }
            }
        }
        out
    }

    fn follow_trails(&self, &position: &(usize, usize)) -> HashSet<(usize, usize)> {
        let (row, col) = position;
        if self[position] == 9 {
            HashSet::from_iter([position])
        } else {
            [
                Some((row + 1, col)),
                row.checked_sub(1).map(|next_row| (next_row, col)),
                Some((row, col + 1)),
                col.checked_sub(1).map(|next_col| (row, next_col)),
            ]
            .into_iter()
            .flatten()
            .filter(Self::in_bounds)
            .filter(|&next| self[next] == self[position] + 1)
            .flat_map(|next| self.follow_trails(&next))
            .collect()
        }
    }

    pub fn trail_scores(&self) -> Vec<usize> {
        self.trailheads()
            .into_iter()
            .map(|trailhead| self.follow_trails(&trailhead).iter().len())
            .collect()
    }

    fn follow_trails_2(&self, &position: &(usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = position;
        if self[position] == 9 {
            vec![position]
        } else {
            [
                Some((row + 1, col)),
                row.checked_sub(1).map(|next_row| (next_row, col)),
                Some((row, col + 1)),
                col.checked_sub(1).map(|next_col| (row, next_col)),
            ]
            .into_iter()
            .flatten()
            .filter(Self::in_bounds)
            .filter(|&next| self[next] == self[position] + 1)
            .flat_map(|next| self.follow_trails_2(&next))
            .collect()
        }
    }

    pub fn trail_scores_2(&self) -> Vec<usize> {
        self.trailheads()
            .into_iter()
            .map(|trailhead| self.follow_trails_2(&trailhead).iter().len())
            .collect()
    }
}

pub fn read_input<P>(path: P) -> Map
where
    P: AsRef<std::path::Path>,
{
    let lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    Map(lines
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse().unwrap())
                .collect()
        })
        .collect())
}
