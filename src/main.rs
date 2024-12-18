mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let start_time = std::time::Instant::now();
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

    // Day 05
    println!(
        "Day 05 Printing Orders: {}",
        day05::day05::check_printing_orders_from_file("./src/data/day05.txt")
    );
    println!(
        "Day 05 Reorder Printing Orders: {}",
        day05::day05::reorder_printing_orders_from_file("./src/data/day05.txt")
    );

    // Day 06
    println!(
        "Day 06 Map: {}",
        day06::day06::count_visited_from_file("./src/data/day06.txt")
    );
    println!(
        "Day 06 Set Obstacles: {}",
        day06::day06::set_obstacles_from_file("./src/data/day06.txt")
    );

    // Day 07
    println!(
        "Day 07 True Equations: {}",
        day07::day07::find_true_equations_from_file("./src/data/day07.txt")
    );
    // Day 07
    println!(
        "Day 07 True Equations 2: {}",
        day07::day07::find_true_equations_two_from_file("./src/data/day07.txt")
    );

    // Day 08
    println!(
        "Day 08 Antinodes: {}",
        day08::day08::get_antinodes_from_file("./src/data/day08.txt")
    );
    println!(
        "Day 08 Antinodes Row: {}",
        day08::day08::get_antinodes_row_from_file("./src/data/day08.txt")
    );

    // Day 09
    println!(
        "Day 09 Checksum: {}",
        day09::day09::calculate_checksum_from_file("./src/data/day09.txt")
    );
    println!(
        "Day 09 Checksum Refragged: {}",
        day09::day09::calculate_checksum_refragged_from_file("./src/data/day09.txt")
    );

    println!("Time: {:?}", start_time.elapsed());
}
