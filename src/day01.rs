pub mod day01 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_to_lists(filename: &str) -> (Vec<usize>, Vec<usize>) {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut a: Vec<usize> = Vec::new();
        let mut b: Vec<usize> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let mut parts = buffer.split_whitespace();
                    a.push(parts.next().unwrap().parse().unwrap());
                    b.push(parts.next().unwrap().parse().unwrap());
                }
                Err(_) => break,
            }
        }
        (a, b)
    }

    fn distance(mut a: Vec<usize>, mut b: Vec<usize>) -> usize {
        assert_eq!(a.len(), b.len());

        a.sort();
        b.sort();

        a.iter().zip(b.iter()).map(|(x, y)| x.abs_diff(*y)).sum()
    }

    fn similarity(a: Vec<usize>, b: Vec<usize>) -> usize {
        a.iter()
            .map(|x| b.iter().filter(|y| y == &x).count() * x)
            .sum()
    }

    pub fn distance_from_file(filename: &str) -> usize {
        let (a, b) = read_to_lists(filename);
        distance(a, b)
    }

    pub fn similarity_from_file(filename: &str) -> usize {
        let (a, b) = read_to_lists(filename);
        similarity(a, b)
    }
}
