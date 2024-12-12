pub mod day05 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        result,
    };

    fn read_to_lists(filename: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut rules: Vec<(usize, usize)> = Vec::new();

        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let split: Vec<&str> = buffer.split("|").collect();
                    if split.len() != 2 {
                        break;
                    }
                    let a: usize = split[0].trim().parse().unwrap();
                    let b: usize = split[1].trim().parse().unwrap();
                    rules.push((a, b));
                }
                Err(_) => break,
            }
        }
        let mut orders: Vec<Vec<usize>> = Vec::new();
        loop {
            let mut buffer = String::new();
            let line = reader.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    let split: Vec<&str> = buffer.split(",").collect();
                    assert!(split.len() >= 1);
                    orders.push(
                        split
                            .into_iter()
                            .map(|s| s.trim().parse().unwrap())
                            .collect(),
                    );
                }
                Err(_) => break,
            }
        }
        (rules, orders)
    }

    pub fn check_printing_orders_from_file(filename: &str) -> usize {
        let (rules, orders) = read_to_lists(filename);

        let mut result = 0;

        for order in orders {
            result += check_order(&order, &rules)
        }
        result
    }

    fn check_order(order: &Vec<usize>, rules: &Vec<(usize, usize)>) -> usize {
        for i in 0..order.len() {
            let rules_apply: Vec<&(usize, usize)> =
                rules.iter().filter(|(x, _)| *x == order[i]).collect();
            let slice = &order[0..i];
            for (_, rule) in rules_apply {
                if slice.contains(&rule) {
                    return 0;
                }
            }
        }

        return order[order.len() / 2];
    }

    pub fn reorder_printing_orders_from_file(filename: &str) -> usize {
        let (rules, orders) = read_to_lists(filename);

        let wrong_orders: Vec<Vec<usize>> = orders
            .iter()
            .filter(|order| check_order(order, &rules) == 0)
            .map(|order| order.clone())
            .collect();

        let mut result = 0;
        for order in wrong_orders {
            result += reorder(&order, &rules);
        }
        result
    }

    fn reorder(order: &Vec<usize>, rules: &Vec<(usize, usize)>) -> usize {
        let rules_apply: Vec<(usize, usize)> = rules
            .iter()
            .filter(|(x, y)| order.contains(x) && order.contains(y))
            .map(|x| *x)
            .collect();

        let mut new_order: Vec<usize> = Vec::new();

        for i in 0..order.len() {
            for pos in 0..=new_order.len() {
                new_order.insert(pos, order[i]);
                if check_order(&new_order, &rules_apply) != 0 {
                    break;
                }
                new_order.remove(pos);
            }
        }
        new_order[new_order.len() / 2]
    }
}
