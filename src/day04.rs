pub mod day04 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_to_lists(filename: &str) -> Vec<Vec<char>> {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut result: Vec<String> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    result.push(buffer);
                }
                Err(_) => break,
            }
        }
        result.into_iter().map(|s| s.chars().collect()).collect()
    }

    pub fn find_xmas_in_file(filename: &str) -> usize {
        let data: Vec<Vec<char>> = read_to_lists(filename);

        // First find all Xs
        let mut result = 0;
        for x in 0..data.len() {
            for y in 0..data[x].len() {
                if data[x][y] == 'X' {
                    result += find_at(&data, x, y);
                }
            }
        }
        result
    }

    fn find_at(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
        let term = "XMAS".chars().collect::<Vec<char>>();
        let x_range = (0.max(x as isize - 1) as usize)..=(data[x].len() - 2).min(x + 1);
        let y_range = (0.max(y as isize - 1) as usize)..=(data.len() - 1).min(y + 1);

        let mut result = 0;

        for i in x_range {
            for j in y_range.clone() {
                if term[1] == data[i][j] {
                    // directional vector
                    let x_vec = i as isize - x as isize;
                    let y_vec = j as isize - y as isize;

                    let mut found = true;

                    for i in 0..term.len() {
                        let x_test = x as isize + x_vec * i as isize;
                        let y_test = y as isize + y_vec * i as isize;
                        if x_test >= data.len() as isize
                            || x_test < 0
                            || y_test >= data[x].len() as isize
                            || y_test < 0
                        {
                            found = false;
                            break;
                        }
                        if term[i] != data[x_test as usize][y_test as usize] {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        result += 1;
                    }
                }
            }
        }
        result
    }

    pub fn find_x_mas_in_file(filename: &str) -> usize {
        let data: Vec<Vec<char>> = read_to_lists(filename);

        let mut result = 0;
        for x in 0..data.len() {
            for y in 0..data[x].len() {
                if data[x][y] == 'A' {
                    result += find_cross_at(&data, x, y);
                }
            }
        }
        result
    }

    fn find_cross_at(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
        if x as isize - 1 < 0
            || x + 1 >= (data[x].len() - 1)
            || y as isize - 1 < 0
            || y + 1 >= data.len()
        {
            return 0;
        }

        let (a, b) = (data[x - 1][y - 1], data[x + 1][y + 1]);
        match (a, b) {
            ('M', 'S') | ('S', 'M') => (),
            _ => return 0,
        }
        let (a, b) = (data[x - 1][y + 1], data[x + 1][y - 1]);
        match (a, b) {
            ('M', 'S') | ('S', 'M') => return 1,
            _ => return 0,
        }
    }
}
