use day9::*;

fn main() {
    let mut disk = read_input("input.txt");

    refrag(&mut disk);
    println!("{}", checksum(&disk))
}
