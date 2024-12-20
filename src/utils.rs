pub mod utils {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn all_directions_from<F>(
        (x, y): (usize, usize),
        map: &Vec<Vec<F>>,
    ) -> Vec<(usize, usize)> {
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

    pub fn read_map(filename: &str) -> Vec<Vec<char>> {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut map: Vec<Vec<char>> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    let row = buffer.trim().chars().into_iter().collect::<Vec<char>>();
                    map.push(row);
                }
            }
        }
        map
    }
}
