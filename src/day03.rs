pub mod day03 {
    use regex::Regex;
    use std::{fs::File, io::Read, str::FromStr};

    struct Mul(usize, usize);

    impl FromStr for Mul {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (x, y) = s
                .strip_prefix("mul(")
                .and_then(|s| s.strip_suffix(")"))
                .and_then(|s| s.split_once(","))
                .ok_or(())?;

            let a = x.parse::<usize>().map_err(|_| ())?;
            let b = y.parse::<usize>().map_err(|_| ())?;
            Ok(Mul(a, b))
        }
    }

    impl Mul {
        fn multiply(&self) -> usize {
            self.0 * self.1
        }
    }

    fn read_input(filename: &str) -> String {
        let mut file = File::open(filename).expect("File not found");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Could not read file");
        buffer
    }

    pub fn multiply_from_file(filename: &str) -> usize {
        let data = read_input(filename);
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

        re.captures_iter(&data)
            .map(|c| c.extract::<0>())
            .map(|(x, _)| Mul::from_str(x).unwrap().multiply())
            .sum()
    }
    pub fn multiply_with_instructions_from_file(filename: &str) -> usize {
        let data = read_input(filename);
        let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();

        let mut blocked = false;
        let mut result = 0;
        for (s, _) in re.captures_iter(&data).map(|c| c.extract::<0>()) {
            match s {
                "do()" => blocked = false,
                "don't()" => blocked = true,
                _ => {
                    if !blocked {
                        result += Mul::from_str(s).unwrap().multiply();
                    }
                }
            }
        }
        result
    }
}
