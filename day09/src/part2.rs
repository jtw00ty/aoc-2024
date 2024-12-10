use std::time::Instant;

use day9::*;

fn main() {
    let start = Instant::now();
    let mut disk = read_input("evil-input.txt");
    defrag(&mut disk);
    let check = checksum(&disk);
    let time = start.elapsed();
    println!("got {} in {time:?}", check)
}
