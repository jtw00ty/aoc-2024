use day6::*;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let mut map = Map::from_file("input.txt");
    map.run_route().await;
    let finish = std::time::Instant::now();
    let out = map.count_visited();
    println!("got result {} in {} us", out, (finish - start).as_micros())
}
