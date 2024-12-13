pub mod day07 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_equations(filename: &str) -> Vec<(usize, Vec<usize>)> {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let result_equation = buffer.split(":").collect::<Vec<&str>>();
                    let result: usize = result_equation[0].parse().unwrap();

                    let equation = result_equation[1].trim().split(" ").collect::<Vec<&str>>();
                    let equation: Vec<usize> =
                        equation.into_iter().map(|x| x.parse().unwrap()).collect();

                    equations.push((result, equation));
                }
                Err(_) => break,
            }
        }
        equations
    }

    pub fn find_true_equations_from_file(filename: &str) -> usize {
        let equations = read_equations(filename);
        let operations: Vec<fn(usize, usize) -> usize> =
            vec![|x: usize, y: usize| x + y, |x: usize, y: usize| x * y];

        let mut counter = 0;
        for (result, equation) in equations {
            counter += solve_equation(result, &equation, &operations);
        }
        counter
    }
    pub fn find_true_equations_two_from_file(filename: &str) -> usize {
        let equations = read_equations(filename);
        let operations: Vec<fn(usize, usize) -> usize> = vec![
            |x: usize, y: usize| x + y,
            |x: usize, y: usize| x * y,
            |x, y| {
                let string = x.to_string() + &y.to_string();
                string.parse().unwrap()
            },
        ];

        let mut counter = 0;
        for (result, equation) in equations {
            counter += solve_equation(result, &equation, &operations);
        }
        counter
    }

    fn solve_equation(
        result: usize,
        equation: &Vec<usize>,
        operations: &Vec<fn(usize, usize) -> usize>,
    ) -> usize {
        if equation.len() == 0 {
            return 0;
        } else if equation.len() == 1 {
            return if equation[0] == result { result } else { 0 };
        } else {
            let (x, y) = (equation[0], equation[1]);

            for operator in operations.iter() {
                let operator_result = operator(x, y);

                let mut slice = equation[2..].to_vec();
                slice.insert(0, operator_result);

                let test_result = solve_equation(result, &slice, operations);

                if test_result == result {
                    return result;
                }
                if test_result > result {
                    return 0;
                }
            }
        }
        0
    }
}
