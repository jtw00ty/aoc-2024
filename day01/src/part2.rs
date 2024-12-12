use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
fn main() {
    let lists = lists("../input.txt");
    let appearances = appearances_map(&lists.1);
    println!(
        "{}",
        lists
            .0
            .iter()
            .filter(|num| appearances.contains_key(num))
            .map(|num| num * appearances.get(num).unwrap())
            .sum::<u32>()
    )
}

fn appearances_map(list: &Vec<u32>) -> HashMap<u32, u32> {
    let mut out = HashMap::new();
    for num in list {
        match out.get_mut(num) {
            Some(count) => *count += 1,
            None => {
                out.insert(*num, 1);
            }
        }
    }
    out
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
