use regex::Regex;
use std::{fs::File, io::Read};

pub fn read_mem<P>(path: P) -> String
where
    P: AsRef<std::path::Path>,
{
    let mut out = String::new();
    File::open(path).unwrap().read_to_string(&mut out).unwrap();
    out
}

pub fn find_muls(mem: &str) -> Vec<(i32, i32)> {
    let pat = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    pat.find_iter(mem)
        .map(|mul| {
            let mut split = mul.as_str().split(&['(', ')', ',']);
            split.next();
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn do_indices(mem: &str) -> Vec<usize> {
    let dos = Regex::new(r"do\(\)").unwrap();
    let do_matches = dos.find_iter(mem);
    do_matches.map(|m| m.end()).collect()
}

pub fn dont_indices(mem: &str) -> Vec<usize> {
    let donts = Regex::new(r"don't\(\)").unwrap();
    let dont_matches = donts.find_iter(mem);
    dont_matches.map(|m| m.end()).collect()
}

pub fn switch_indices(mem: &str) -> Vec<usize> {
    let dos = do_indices(mem);
    let donts = dont_indices(mem);

    let mut switches = vec![0];
    for parity in 0.. {
        match if parity % 2 == 0 { &donts } else { &dos }
            .iter()
            .find(|index| index > &switches.last().unwrap())
        {
            Some(i) => switches.push(i.clone()),
            None => {
                break;
            }
        }
    }

    switches
}

pub fn enabled_muls(mem: &str) -> Vec<(i32, i32)> {
    let mut out = vec![];

    let mut switches = switch_indices(mem).into_iter();
    while let (Some(start), Some(end)) = (switches.next(), switches.next().or(Some(mem.len()))) {
        out.append(&mut find_muls(&mem[start..end]));
    }

    out
}
