use day9::*;

fn main() {
    let mut disk = read_input("input.txt");

    defrag(&mut disk);
    println!("{}", checksum(&disk))
}
