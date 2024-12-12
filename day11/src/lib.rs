use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Stones(HashMap<u64, u64>);

impl Stones {
    pub fn from_file<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let mut input = String::new();
        File::open(path)
            .unwrap()
            .read_to_string(&mut input)
            .unwrap();
        let mut map = HashMap::new();

        for stone in input.split(' ').map(|num| num.parse::<u64>().unwrap()) {
            *map.entry(stone).or_insert(0) += 1;
        }
        Self(map)
    }

    pub fn len(&self) -> u64 {
        self.0.values().sum()
    }

    pub fn is_empty(&self) -> bool {
        self.0.values().sum::<u64>() == 0
    }

    fn apply_rules(&stone: &u64) -> Vec<u64> {
        if stone == 0 {
            return vec![1];
        }
        let string = stone.to_string();
        let len = string.len();
        if len % 2 == 0 {
            return vec![
                string[..len / 2].parse().unwrap(),
                string[len / 2..].parse().unwrap(),
            ];
        }
        vec![stone * 2024]
    }

    pub fn blink(&mut self) {
        let mut after = HashMap::new();
        for (num, count) in &self.0 {
            for new in Self::apply_rules(num) {
                *after.entry(new).or_insert(0) += count;
            }
        }
        self.0 = after;
    }
}
