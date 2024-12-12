mod day01;
mod day02;
mod day03;
mod day04;

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

    // Day 03
    println!(
        "Day 03 Multiply: {}",
        day03::day03::multiply_from_file("./src/data/day03.txt")
    );
    println!(
        "Day 03 Multiply with Instructions: {}",
        day03::day03::multiply_with_instructions_from_file("./src/data/day03.txt")
    );

    // Day 04
    println!(
        "Day 04 XMAS: {}",
        day04::day04::find_xmas_in_file("./src/data/day04.txt")
    );
    println!(
        "Day 04 X-MAS: {}",
        day04::day04::find_x_mas_in_file("./src/data/day04.txt")
    );
}
