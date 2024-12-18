pub mod day10 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use itertools::Itertools;

    fn read_map(filename: &str) -> Vec<Vec<usize>> {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut map: Vec<Vec<usize>> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    let row = buffer.trim().chars();
                    map.push(row.map(|x| x.to_digit(10).unwrap() as usize).collect());
                }
            }
        }
        map
    }

    fn find_all_in_map(term: usize, map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == term {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn all_directions_from((x, y): (usize, usize), map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < map[y].len() - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < map.len() - 1 {
            result.push((x, y + 1));
        }
        result
    }

    fn hike_trail(
        (x, y): (usize, usize),
        map: &Vec<Vec<usize>>,
        prev_step: usize,
    ) -> Vec<(usize, usize)> {
        if map[y][x] != prev_step + 1 {
            return Vec::new();
        }
        if map[y][x] == 9 && prev_step == 8 {
            return vec![(x, y)];
        }

        let mut result: Vec<(usize, usize)> = Vec::new();
        for direction in all_directions_from((x, y), map) {
            result.append(&mut hike_trail(direction, map, map[y][x]));
        }
        result
    }

    pub fn count_trails_from_file(filename: &str) -> usize {
        let map = read_map(filename);

        let mut trail_count = 0;
        for starting_position in find_all_in_map(0, &map) {
            let mut result: Vec<(usize, usize)> = Vec::new();
            for direction in all_directions_from(starting_position, &map) {
                result.append(&mut hike_trail(direction, &map, 0));
            }
            trail_count += result.into_iter().unique().count();
        }

        trail_count
    }

    pub fn count_all_trails_from_file(filename: &str) -> usize {
        let map = read_map(filename);

        let mut trail_count = 0;
        for starting_position in find_all_in_map(0, &map) {
            let mut result: Vec<(usize, usize)> = Vec::new();
            for direction in all_directions_from(starting_position, &map) {
                result.append(&mut hike_trail(direction, &map, 0));
            }
            trail_count += result.len();
        }

        trail_count
    }
}
