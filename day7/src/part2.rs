use std::time::Instant;

use day7::*;

fn main() {
    let equations = read_input("input.txt");

    let start = Instant::now();
    let out = equations
        .into_iter()
        .filter(|equation| equation.find_operators_concat().is_some())
        .map(|eq| eq.value)
        .sum::<f64>();
    println!("got {out} in {:?}", start.elapsed());
}
