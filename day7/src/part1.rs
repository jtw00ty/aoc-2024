use day7::*;

fn main() {
    let equations = read_input("input.txt");

    println!(
        "{}",
        equations
            .into_iter()
            .filter(|equation| equation.find_operators().is_some())
            .map(|eq| eq.value)
            .sum::<f64>()
    );
}
