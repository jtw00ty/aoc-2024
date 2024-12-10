use day10::*;

fn main() {
    let map = read_input("input.txt");

    println!("{}", map.trail_scores().iter().sum::<usize>())
}
