use day8::*;

fn main() {
    let map = read_input("input.txt");

    let out = find_nodes_harmonic(&map);
    println!("{}", out.len());
}
