pub mod day06 {
    use std::collections::HashSet;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    #[derive(Clone, Copy, PartialEq, Hash, Eq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        fn turn_right(&self) -> Direction {
            match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }

        fn move_from(&self, x: usize, y: usize) -> (isize, isize) {
            match self {
                Direction::North => (x as isize, y as isize - 1),
                Direction::East => (x as isize + 1, y as isize),
                Direction::South => (x as isize, y as isize + 1),
                Direction::West => (x as isize - 1, y as isize),
            }
        }
    }

    fn read_map(filename: &str) -> (Vec<Vec<bool>>, (usize, usize, Direction)) {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut x = 0;
        let mut y = 0;
        let mut direction = Direction::North;

        let mut map: Vec<Vec<bool>> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let mut row: Vec<bool> = Vec::new();
                    for c in buffer.chars() {
                        match c {
                            '#' => row.push(true),
                            '.' => row.push(false),
                            '^' => {
                                x = row.len();
                                y = map.len();
                                row.push(false);
                                direction = Direction::North;
                            }
                            '>' => {
                                x = row.len();
                                y = map.len();
                                row.push(false);
                                direction = Direction::East;
                            }
                            'v' => {
                                x = row.len();
                                y = map.len();
                                row.push(false);
                                direction = Direction::South;
                            }
                            '<' => {
                                x = row.len();
                                y = map.len();
                                row.push(false);
                                direction = Direction::West;
                            }
                            _ => break,
                        }
                    }
                    map.push(row);
                }
                Err(_) => break,
            }
        }

        (map, (x, y, direction))
    }

    fn is_outside(map: &Vec<Vec<bool>>, x: isize, y: isize) -> bool {
        x < 0 || x >= map[0].len() as isize || y < 0 || y >= map.len() as isize
    }

    pub fn count_visited_from_file(filename: &str) -> usize {
        let (map, (mut x, mut y, mut direction)) = read_map(filename);

        let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

        loop {
            visited[y][x] = true;

            // new position if no obstacle ahead
            let (nx, ny) = direction.move_from(x, y);
            if is_outside(&map, nx, ny) {
                // moved outside the map
                break;
            }
            let (nx, ny) = (nx as usize, ny as usize);

            if map[ny][nx] {
                direction = direction.turn_right();
            } else {
                (x, y) = (nx, ny);
            }
        }

        visited.iter().flatten().filter(|x| **x).count()
    }

    pub fn set_obstacles_from_file(filename: &str) -> usize {
        let (map, (mut x, mut y, mut direction)) = read_map(filename);
        let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
        let mut obstacles_counter = 0;

        loop {
            visited[y][x] = true;

            // new position if no obstacle ahead
            let (nx, ny) = direction.move_from(x, y);
            if is_outside(&map, nx, ny) {
                // moved outside the map
                break;
            }
            let (nx, ny) = (nx as usize, ny as usize);

            if map[ny][nx] {
                direction = direction.turn_right();
            } else {
                if !visited[ny][nx] && has_loop_with_obstacle_at(&map, (nx, ny), (x, y), direction)
                {
                    obstacles_counter += 1;
                }

                (x, y) = (nx, ny);
            }
        }
        obstacles_counter
    }

    fn has_loop_with_obstacle_at(
        map: &Vec<Vec<bool>>,
        obstacle: (usize, usize),
        (mut x, mut y): (usize, usize),
        mut direction: Direction,
    ) -> bool {
        let mut hits: HashSet<((usize, usize), Direction)> = HashSet::new();

        loop {
            // new position if no obstacle ahead
            let (nx, ny) = direction.move_from(x, y);
            if is_outside(&map, nx, ny) {
                // moved outside the map
                return false;
            }
            let (nx, ny) = (nx as usize, ny as usize);

            if map[ny][nx] || obstacle == (nx, ny) {
                if hits.contains(&((nx, ny), direction)) {
                    return true;
                }
                hits.insert(((nx, ny), direction));
                direction = direction.turn_right();
            } else {
                (x, y) = (nx, ny);
            }
        }
    }
}
