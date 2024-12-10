use day6::*;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let mut map = Map::from_file("input.txt");
    map.run_route().await;
    let out = map.count_visited();
    let finish = start.elapsed();
    println!("got result {} in {:?}", out, finish)
}
