use day6::*;

#[tokio::main]
async fn main() {
    let mut map = Map::from_file("input.txt");
    println!("{}", map.loop_obstacles().await.len())
}
