use day10::*;

fn main() {
    let map = read_input("input.txt");

    println!("{}", map.trail_scores_2().iter().sum::<usize>())
}
