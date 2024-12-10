use day5::*;

fn main() {
    let (rules, revisons) = read_input("input.txt");

    println!(
        "{}",
        revisons
            .into_iter()
            .filter(|revision| !valid_revision(revision, &rules))
            .map(|revision| correct_revision(&revision, &rules))
            .map(|revision| revision.get(revision.len() / 2).cloned().unwrap())
            .sum::<u16>()
    )
}
