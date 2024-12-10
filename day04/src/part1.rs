use day4::*;

fn main() {
    let word_search = read_word_search("input.txt");
    println!("{}", num_xmas(&word_search));
}
