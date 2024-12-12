use day11::*;

fn main() {
    let mut stones = Stones::from_file("input.txt");
    for _ in 0..25 {
        stones.blink();
    }

    println!("{}", stones.len())
}
