use day2::*;

fn main() {
    let lines = get_lines("input.txt");
    println!(
        "{}",
        lines
            .iter()
            .filter(|report| is_safe_dampened(report))
            .count()
    )
}
