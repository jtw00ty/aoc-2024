use std::fs::File;
use std::io::{self, BufRead};

pub fn read_word_search<P>(path: P) -> Vec<Vec<char>>
where
    P: AsRef<std::path::Path>,
{
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
    lines.map(|line| line.unwrap().chars().collect()).collect()
}

pub fn find_char(char: &char, word_search: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for (i, row) in word_search.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if char == c {
                out.push((i, j));
            }
        }
    }
    out
}

pub fn check_xmas(word_search: &Vec<Vec<char>>, position: (usize, usize)) -> usize {
    let mut total = 0;

    // ->
    if let (Some('M'), Some('A'), Some('S')) = (
        word_search
            .get(position.0)
            .and_then(|row| row.get(position.1 + 1)),
        word_search
            .get(position.0)
            .and_then(|row| row.get(position.1 + 2)),
        word_search
            .get(position.0)
            .and_then(|row| row.get(position.1 + 3)),
    ) {
        total += 1;
    }

    // <-
    if position.1 >= 3 {
        if let (Some('M'), Some('A'), Some('S')) = (
            word_search
                .get(position.0)
                .and_then(|row| row.get(position.1 - 1)),
            word_search
                .get(position.0)
                .and_then(|row| row.get(position.1 - 2)),
            word_search
                .get(position.0)
                .and_then(|row| row.get(position.1 - 3)),
        ) {
            total += 1;
        }
    }

    // |
    // v
    if let (Some('M'), Some('A'), Some('S')) = (
        word_search
            .get(position.0 + 1)
            .and_then(|row| row.get(position.1)),
        word_search
            .get(position.0 + 2)
            .and_then(|row| row.get(position.1)),
        word_search
            .get(position.0 + 3)
            .and_then(|row| row.get(position.1)),
    ) {
        total += 1;
    }

    // ^
    // |
    if position.0 >= 3 {
        if let (Some('M'), Some('A'), Some('S')) = (
            word_search
                .get(position.0 - 1)
                .and_then(|row| row.get(position.1)),
            word_search
                .get(position.0 - 2)
                .and_then(|row| row.get(position.1)),
            word_search
                .get(position.0 - 3)
                .and_then(|row| row.get(position.1)),
        ) {
            total += 1;
        }
    }

    // |
    // +->
    if let (Some('M'), Some('A'), Some('S')) = (
        word_search
            .get(position.0 + 1)
            .and_then(|row| row.get(position.1 + 1)),
        word_search
            .get(position.0 + 2)
            .and_then(|row| row.get(position.1 + 2)),
        word_search
            .get(position.0 + 3)
            .and_then(|row| row.get(position.1 + 3)),
    ) {
        total += 1;
    }

    // <-+
    //   |

    if position.0 >= 3 && position.1 >= 3 {
        if let (Some('M'), Some('A'), Some('S')) = (
            word_search
                .get(position.0 - 1)
                .and_then(|row| row.get(position.1 - 1)),
            word_search
                .get(position.0 - 2)
                .and_then(|row| row.get(position.1 - 2)),
            word_search
                .get(position.0 - 3)
                .and_then(|row| row.get(position.1 - 3)),
        ) {
            total += 1;
        }
    }

    // +->
    // |
    if position.0 >= 3 {
        if let (Some('M'), Some('A'), Some('S')) = (
            word_search
                .get(position.0 - 1)
                .and_then(|row| row.get(position.1 + 1)),
            word_search
                .get(position.0 - 2)
                .and_then(|row| row.get(position.1 + 2)),
            word_search
                .get(position.0 - 3)
                .and_then(|row| row.get(position.1 + 3)),
        ) {
            total += 1;
        }
    }
    //    |
    // < -+

    if position.1 >= 3 {
        if let (Some('M'), Some('A'), Some('S')) = (
            word_search
                .get(position.0 + 1)
                .and_then(|row| row.get(position.1 - 1)),
            word_search
                .get(position.0 + 2)
                .and_then(|row| row.get(position.1 - 2)),
            word_search
                .get(position.0 + 3)
                .and_then(|row| row.get(position.1 - 3)),
        ) {
            total += 1;
        }
    }

    total
}

pub fn num_xmas(word_search: &Vec<Vec<char>>) -> usize {
    find_char(&'X', word_search)
        .into_iter()
        .map(|x| check_xmas(word_search, x))
        .sum()
}

pub fn check_x_mas(word_search: &Vec<Vec<char>>, position: &(usize, usize)) -> bool {
    const CORRECT_CORNERS: [(&char, &char, &char, &char); 4] = [
        (&'M', &'M', &'S', &'S'),
        (&'S', &'M', &'M', &'S'),
        (&'S', &'S', &'M', &'M'),
        (&'M', &'S', &'S', &'M'),
    ];

    let corners = (
        word_search
            .get(position.0 - 1)
            .and_then(|row| row.get(position.1 - 1))
            .unwrap(),
        word_search
            .get(position.0 - 1)
            .and_then(|row| row.get(position.1 + 1))
            .unwrap(),
        word_search
            .get(position.0 + 1)
            .and_then(|row| row.get(position.1 + 1))
            .unwrap(),
        word_search
            .get(position.0 + 1)
            .and_then(|row| row.get(position.1 - 1))
            .unwrap(),
    );
    CORRECT_CORNERS.contains(&corners)
}

pub fn num_x_mas(word_search: &Vec<Vec<char>>) -> usize {
    find_char(&'A', word_search)
        .into_iter()
        .filter(|(row, col)| ![0, 139].contains(row) && ![0, 139].contains(col))
        .filter(|position| check_x_mas(word_search, position))
        .count()
}
