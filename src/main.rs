use day01::day01::{distance_from_file, similarity_from_file};

mod day01;

fn main() {
    println!(
        "Day 01 Distance: {}",
        distance_from_file("./src/data/day01.txt")
    );
    println!(
        "Day 01 Similarity: {}",
        similarity_from_file("./src/data/day01.txt")
    );
}
