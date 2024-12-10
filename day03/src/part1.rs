use day3::*;

fn main() {
    println!(
        "{}",
        find_muls(&read_mem("input.txt"))
            .iter()
            .map(|(first, second)| first * second)
            .sum::<i32>()
    );
}
