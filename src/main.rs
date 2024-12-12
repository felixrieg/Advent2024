mod day01;
mod day02;

fn main() {
    // Day 01
    println!(
        "Day 01 Distance: {}",
        day01::day01::distance_from_file("./src/data/day01.txt")
    );
    println!(
        "Day 01 Similarity: {}",
        day01::day01::similarity_from_file("./src/data/day01.txt")
    );

    // Day 02
    println!(
        "Day 02 Reports: {}",
        day02::day02::check_reports_from_file("./src/data/day02.txt")
    );
    println!(
        "Day 02 Tolerate Reports: {}",
        day02::day02::check_tolerate_reports_from_file("./src/data/day02.txt")
    );
}
