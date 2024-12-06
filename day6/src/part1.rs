use day6::*;

#[tokio::main]
async fn main() {
    let mut map = Map::from_file("input.txt");
    map.run_route();
    println!("{}", map.count_visited());
}
