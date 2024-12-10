use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut lists = lists("input.txt");
    lists.0.sort_unstable();
    lists.1.sort_unstable();

    println!(
        "{}",
        lists
            .0
            .iter()
            .zip(lists.1)
            .map(|(first, second)| { first.abs_diff(second) })
            .sum::<u32>()
    )
}

fn lists<P>(path: P) -> (Vec<u32>, Vec<u32>)
where
    P: AsRef<std::path::Path>,
{
    let mut out = (vec![], vec![]);
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
    let pairs = lines.map(|line| {
        line.unwrap()
            .split_once("   ")
            .map(|(first, second)| {
                (
                    first.parse::<u32>().unwrap(),
                    second.parse::<u32>().unwrap(),
                )
            })
            .unwrap()
    });
    for pair in pairs {
        out.0.push(pair.0);
        out.1.push(pair.1);
    }
    out
}
