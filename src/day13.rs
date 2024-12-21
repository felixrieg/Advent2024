pub mod day13 {
    use std::{fs::File, io::Read};

    use regex::Regex;

    fn read_equations_from_file(
        filename: &str,
    ) -> Vec<((usize, usize), (usize, usize), (usize, usize))> {
        let mut file = File::open(filename).expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Could not read file");

        let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)\n").unwrap();

        let mut results: Vec<((usize, usize), (usize, usize), (usize, usize))> = vec![];
        for (_, [a_x, a_y, b_x, b_y, prize_x, prize_y]) in
            re.captures_iter(&data).map(|c| c.extract())
        {
            results.push((
                (a_x.parse().unwrap(), a_y.parse().unwrap()),
                (b_x.parse().unwrap(), b_y.parse().unwrap()),
                (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
            ));
        }
        results
    }
    fn solve_equation(equation: ((usize, usize), (usize, usize), (usize, usize))) -> usize {
        let ((a_x, a_y), (b_x, b_y), (prize_x, prize_y)) = equation;

        let a_x = a_x as f64;
        let a_y = a_y as f64;
        let b_x = b_x as f64;
        let b_y = b_y as f64;
        let prize_x = prize_x as f64;
        let prize_y = prize_y as f64;

        let b = (a_x * prize_y - a_y * prize_x) / (a_x * b_y - a_y * b_x);
        let a_1 = (prize_x - b_x * b) / a_x;
        let a_2 = (prize_y - b_y * b) / a_y;

        if a_1 == a_2 && a_1 >= 0.0 && b >= 0.0 {
            if a_1.fract() == 0.0 && b.fract() == 0.0 {
                return (a_1 * 3.0 + b * 1.0) as usize;
            }
        }
        0
    }

    pub fn solve_claw_machines(filename: &str) -> usize {
        let equations = read_equations_from_file(filename);
        equations
            .into_iter()
            .map(|equation| solve_equation(equation))
            .sum()
    }
    pub fn solve_multiplied_claw_machines(filename: &str) -> usize {
        let equations = read_equations_from_file(filename);
        equations
            .into_iter()
            .map(|(a, b, (x, y))| solve_equation((a, b, (x + 10000000000000, y + 10000000000000))))
            .sum()
    }
}
