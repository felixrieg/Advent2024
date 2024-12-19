pub mod day11 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_to_lists(filename: &str) -> Vec<usize> {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    return buffer
                        .trim()
                        .split(" ")
                        .into_iter()
                        .map(|number| number.parse().unwrap())
                        .collect();
                }
                Err(_) => break,
            }
        }
        Vec::new()
    }

    pub fn apply_rule(number: usize) -> Vec<usize> {
        match number {
            0 => vec![1],
            x if x.to_string().len() % 2 == 0 => {
                let left_part = x
                    .to_string()
                    .chars()
                    .take(x.to_string().len() / 2)
                    .collect::<String>();
                let right_part = x
                    .to_string()
                    .chars()
                    .skip(x.to_string().len() / 2)
                    .collect::<String>();
                vec![left_part.parse().unwrap(), right_part.parse().unwrap()]
            }
            x => vec![x * 2024],
        }
    }

    fn apply_rule_with_cache(
        number: usize,
        n: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if n <= 0 {
            return 1;
        }

        if cache.contains_key(&(number, n)) {
            return *cache.get(&(number, n)).unwrap();
        }

        // no fitting cache found
        let result = vec![number]
            .into_iter()
            .map(|number| apply_rule(number))
            .flatten()
            .map(|number| apply_rule_with_cache(number, n - 1, cache))
            .sum();
        cache.insert((number, n), result);

        result
    }

    pub fn blink_n_times(filename: &str, n: usize) -> usize {
        let position = read_to_lists(filename);
        let mut cache = HashMap::new();
        let x = position
            .into_iter()
            .map(|x| apply_rule_with_cache(x, n, &mut cache))
            .sum();
        x
    }
}
