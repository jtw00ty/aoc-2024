use day9::*;

fn main() {
    let disk = read_input("input.txt");

    println!("{}", refrag_checksum(&disk))
}
