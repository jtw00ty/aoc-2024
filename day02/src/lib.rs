use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines<P>(input_file: P) -> Vec<Vec<i16>>
where
    P: AsRef<std::path::Path>,
{
    let lines = io::BufReader::new(File::open(input_file).unwrap()).lines();
    lines
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|str| str.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_safe(report: &Vec<i16>) -> bool {
    let mut pairs = report.iter().zip(report.iter().skip(1));
    let ascending = pairs
        .clone()
        .next()
        .map(|(first, second)| first < second)
        .unwrap_or_default();
    pairs.all(|(first, second)| {
        let diff = second - first;
        if ascending {
            diff > 0 && diff <= 3
        } else {
            diff < 0 && diff >= -3
        }
    })
}

pub fn is_safe_dampened(report: &Vec<i16>) -> bool {
    for i in 0..report.len() {
        let mut candidate = report.clone();
        candidate.remove(i);
        if is_safe(&candidate) {
            return true;
        }
    }
    false
}
