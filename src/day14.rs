pub mod day14 {
    use std::{fs::File, io::Read};

    use regex::Regex;

    const MAX_X: usize = 101;
    const MAX_Y: usize = 103;

    fn read_robots_from_file(filename: &str) -> Vec<((usize, usize), (isize, isize))> {
        let mut file = File::open(filename).expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Could not read file");

        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        let mut results: Vec<((usize, usize), (isize, isize))> = vec![];
        for (_, [p_x, p_y, v_x, v_y]) in re.captures_iter(&data).map(|c| c.extract()) {
            results.push((
                (p_x.parse().unwrap(), p_y.parse().unwrap()),
                (v_x.parse().unwrap(), v_y.parse().unwrap()),
            ));
        }
        results
    }

    fn move_robot(robot: ((usize, usize), (isize, isize))) -> ((usize, usize), (isize, isize)) {
        let ((x, y), (dx, dy)) = robot;
        (
            (
                ((x as isize + dx + MAX_X as isize) % MAX_X as isize) as usize,
                ((y as isize + dy + MAX_Y as isize) % MAX_Y as isize) as usize,
            ),
            (dx, dy),
        )
    }

    fn move_robot_n_times(
        robot: ((usize, usize), (isize, isize)),
        n: usize,
    ) -> ((usize, usize), (isize, isize)) {
        let mut robot = robot;
        for _ in 0..n {
            robot = move_robot(robot);
        }
        robot
    }

    fn calc_quadrant(robot: &((usize, usize), (isize, isize))) -> (usize, usize, usize, usize) {
        let ((x, y), _) = robot;
        let x = *x;
        let y = *y;
        if x < MAX_X / 2 && y < MAX_Y / 2 {
            (1, 0, 0, 0)
        } else if x > MAX_X / 2 && y < MAX_Y / 2 {
            (0, 1, 0, 0)
        } else if x < MAX_X / 2 && y > MAX_Y / 2 {
            (0, 0, 1, 0)
        } else if x > MAX_X / 2 && y > MAX_Y / 2 {
            (0, 0, 0, 1)
        } else {
            (0, 0, 0, 0)
        }
    }

    fn calculate_safety_factor(robots: &Vec<((usize, usize), (isize, isize))>) -> usize {
        let result = robots
            .into_iter()
            .map(|robot| calc_quadrant(robot))
            .fold((0, 0, 0, 0), |a, b| {
                (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
            });

        result.0 * result.1 * result.2 * result.3
    }

    fn print_robots(robots: &Vec<((usize, usize), (isize, isize))>, n: usize) {
        let mut map = vec![vec![" "; MAX_X]; MAX_Y];
        for ((x, y), _) in robots {
            map[*y][*x] = "X";
        }
        for row in map {
            println!("{}", row.join(""));
        }
    }

    pub fn calc_robot_position(filename: &str) -> usize {
        let mut robots = read_robots_from_file(filename);

        robots = robots
            .into_iter()
            .map(|robot| move_robot_n_times(robot, 100))
            .collect();

        calculate_safety_factor(&robots)
    }

    pub fn print_christmas_tree(filename: &str, print_tree: bool) -> usize {
        let mut initial_robots = read_robots_from_file(filename);
        let mut robots = initial_robots.clone();

        let mut min_safety_factor = 0;
        let mut iteration = 0;

        for i in 1..(MAX_X * MAX_Y) {
            robots = robots.into_iter().map(|robot| move_robot(robot)).collect();
            let new_safety_factor = calculate_safety_factor(&robots);
            if new_safety_factor <= min_safety_factor || min_safety_factor == 0 {
                min_safety_factor = new_safety_factor;
                iteration = i;
            }
        }

        if print_tree {
            for _ in 0..iteration {
                initial_robots = initial_robots
                    .into_iter()
                    .map(|robot| move_robot(robot))
                    .collect();
            }
            print_robots(&initial_robots, iteration);
        }
        iteration
    }
}
