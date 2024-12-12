use day11::*;

#[tokio::main]
async fn main() {
    let mut stones = Stones::from_file("input.txt");
    for _ in 0..75 {
        stones.blink();
    }

    println!("{}", stones.len());
}
