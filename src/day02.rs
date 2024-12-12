pub mod day02 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_to_lists(filename: &str) -> Vec<Vec<usize>> {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut result: Vec<Vec<usize>> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let mut parts = buffer.split_whitespace();
                    let mut row: Vec<usize> = Vec::new();
                    loop {
                        match parts.next() {
                            Some(x) => row.push(x.parse().unwrap()),
                            None => break,
                        }
                    }
                    result.push(row);
                }
                Err(_) => break,
            }
        }
        result
    }

    pub fn check_reports_from_file(filename: &str) -> usize {
        let data = read_to_lists(filename);

        data.iter()
            .map(|report| check_report(report))
            .filter(|x| *x)
            .count()
    }

    fn check_report(report: &Vec<usize>) -> bool {
        let mut increasing = true;
        for i in 0..(report.len() - 1) {
            if i == 0 {
                if report[i] > report[i + 1] {
                    increasing = false;
                }
            }
            if report[i] > report[i + 1] && increasing {
                return false;
            }
            if report[i] < report[i + 1] && !increasing {
                return false;
            }

            if report[i] == report[i + 1] {
                return false;
            }

            if report[i].abs_diff(report[i + 1]) > 3 {
                return false;
            }
        }
        true
    }
    fn check_tolerate_report(report: &Vec<usize>) -> bool {
        if check_report(report) {
            return true;
        }

        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if check_report(&report) {
                return true;
            }
        }
        false
    }

    pub fn check_tolerate_reports_from_file(filename: &str) -> usize {
        let data = read_to_lists(filename);

        data.iter()
            .map(|report| check_tolerate_report(report))
            .filter(|x| *x)
            .count()
    }
}
