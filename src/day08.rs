pub mod day08 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_to_lists(filename: &str) -> HashMap<char, Vec<(usize, usize)>> {
        let mut result: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut y = 0;
        let mut max_x = 0;

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let mut x = 0;

                    for c in buffer.chars() {
                        match c {
                            '.' | '\n' => x += 1,
                            c => {
                                if result.contains_key(&c) {
                                    let mut list = result.get(&c).unwrap().clone();
                                    list.push((x, y));
                                    result.insert(c, list);
                                } else {
                                    result.insert(c, vec![(x, y)]);
                                }
                                x += 1;
                            }
                        }
                    }
                    max_x = x;
                }
                Err(_) => break,
            }
            y += 1;
        }

        result.insert('.', vec![(max_x - 2, y - 1)]);
        result
    }

    pub fn get_antinodes_from_file(filename: &str) -> usize {
        let antennas = read_to_lists(filename);

        let (max_x, max_y) = antennas.get(&'.').unwrap()[0];
        let mut field = vec![vec![false; max_x + 1]; max_y + 1];

        for (_, list) in antennas.iter() {
            for i in 0..list.len() {
                for j in i + 1..list.len() {
                    let (x1, y1) = list[i];
                    let (x2, y2) = list[j];

                    let diff_x = x2 as isize - x1 as isize;
                    let diff_y = y2 as isize - y1 as isize;

                    let x = x1 as isize - diff_x;
                    let y = y1 as isize - diff_y;

                    if x >= 0 && x <= max_x as isize && y >= 0 && y <= max_y as isize {
                        field[y as usize][x as usize] = true;
                    }

                    let x = x2 as isize + diff_x;
                    let y = y2 as isize + diff_y;

                    if x >= 0 && x <= max_x as isize && y >= 0 && y <= max_y as isize {
                        field[y as usize][x as usize] = true;
                    }
                }
            }
        }

        field.iter().flatten().filter(|&&x| x).count()
    }

    pub fn get_antinodes_row_from_file(filename: &str) -> usize {
        let antennas = read_to_lists(filename);

        let (max_x, max_y) = antennas.get(&'.').unwrap()[0];
        let mut field = vec![vec![false; max_x + 1]; max_y + 1];

        for (_, list) in antennas.iter() {
            for i in 0..list.len() {
                for j in i + 1..list.len() {
                    let (x1, y1) = list[i];
                    let (x2, y2) = list[j];

                    field[y1][x1] = true;
                    field[y2][x2] = true;

                    let diff_x = x2 as isize - x1 as isize;
                    let diff_y = y2 as isize - y1 as isize;

                    let mut x = x1 as isize;
                    let mut y = y1 as isize;

                    loop {
                        x = x - diff_x;
                        y = y - diff_y;

                        if x >= 0 && x <= max_x as isize && y >= 0 && y <= max_y as isize {
                            field[y as usize][x as usize] = true;
                        } else {
                            break;
                        }
                    }

                    let mut x = x2 as isize;
                    let mut y = y2 as isize;

                    loop {
                        x = x + diff_x;
                        y = y + diff_y;

                        if x >= 0 && x <= max_x as isize && y >= 0 && y <= max_y as isize {
                            field[y as usize][x as usize] = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        field.iter().flatten().filter(|&&x| x).count()
    }
}
