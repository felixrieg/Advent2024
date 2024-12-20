pub mod day12 {
    use utils::{all_directions_from, read_map};

    use crate::utils::*;
    use std::vec;

    fn get_area_of(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let id = map[y][x];

        let mut result = vec![(x, y)];

        let mut stack = vec![(x, y)];
        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            let directions = all_directions_from((x, y), map);
            for (x, y) in directions.iter() {
                if map[*y][*x] == id && !result.contains(&(*x, *y)) {
                    stack.push((*x, *y));
                    result.push((*x, *y));
                }
            }
        }
        result
    }

    fn calculate_area_size(map: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
        let mut result = vec![vec![0; map[0].len()]; map.len()];
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if result[y][x] != 0 {
                    continue;
                }
                let area = get_area_of(map, (x, y));
                let area_size = area.len();
                for (x, y) in area {
                    result[y][x] = area_size;
                }
            }
        }
        result
    }

    fn calculate_perimeters(map: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
        let mut result = vec![vec![0; map[0].len()]; map.len()];
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let mut perimeter = 4;

                let id = map[y][x];
                let directions = all_directions_from((x, y), map);

                for (x, y) in directions.iter() {
                    if map[*y][*x] == id {
                        perimeter -= 1;
                    }
                }
                result[y][x] = perimeter;
            }
        }
        result
    }

    pub fn calculate_fence_cost(filename: &str) -> usize {
        let map = read_map(filename);
        let area_sizes = calculate_area_size(&map);
        let perimeters = calculate_perimeters(&map);
        area_sizes
            .iter()
            .zip(perimeters.iter())
            .map(|(area, perimeter)| {
                area.iter()
                    .zip(perimeter.iter())
                    .map(|(area, perimeter)| area * perimeter)
                    .sum::<usize>()
            })
            .sum()
    }

    fn all_corners_for<F>((x, y): (usize, usize), map: &Vec<Vec<F>>) -> Vec<Vec<(usize, usize)>> {
        let mut result: Vec<Vec<(usize, usize)>> = vec![vec![(x, y)]; 4];
        let left_top = 0;
        let right_top = 1;
        let left_bottom = 2;
        let right_bottom = 3;
        if x > 0 {
            result[left_top].push((x - 1, y));
            result[left_bottom].push((x - 1, y));
        }
        if x < map[y].len() - 1 {
            result[right_top].push((x + 1, y));
            result[right_bottom].push((x + 1, y));
        }
        if y > 0 {
            result[left_top].push((x, y - 1));
            result[right_top].push((x, y - 1));
        }
        if y < map.len() - 1 {
            result[left_bottom].push((x, y + 1));
            result[right_bottom].push((x, y + 1));
        }
        if x > 0 && y > 0 {
            // left top corner
            result[left_top].push((x - 1, y - 1));
        }
        if x < map[y].len() - 1 && y > 0 {
            // right top corner
            result[right_top].push((x + 1, y - 1));
        }
        if x > 0 && y < map.len() - 1 {
            // left bottom corner
            result[left_bottom].push((x - 1, y + 1));
        }
        if x < map[y].len() - 1 && y < map.len() - 1 {
            // right bottom corner
            result[right_bottom].push((x + 1, y + 1));
        }
        result
    }

    fn calculate_area_sides(area: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> usize {
        let mut sides = 0;
        let mut visited = Vec::new();
        for (x, y) in area.iter() {
            let corners = all_corners_for((*x, *y), map);

            for corner in corners {
                let mut corners = 4;
                let mut already_counted = false;
                for (x, y) in corner.clone() {
                    if area.contains(&(x, y)) {
                        corners -= 1;
                    }
                    if visited.contains(&(x, y)) {
                        corners -= 1;
                        already_counted = true;
                    }
                }
                if !already_counted && corners == 2 {
                    // if only diagonal corners are in the area -> 2 sides
                    let mut nodes = Vec::new();
                    for (x, y) in corner.clone() {
                        if !area.contains(&(x, y)) {
                            nodes.push((x, y));
                        }
                    }
                    if nodes.len() == 2 && nodes[0].0 != nodes[1].0 && nodes[0].1 != nodes[1].1 {
                        sides += 2;
                    }
                }
                if corners % 2 == 1 && !already_counted {
                    sides += 1;
                }
            }
            visited.push((*x, *y));
        }
        sides
    }

    pub fn calculate_discounted_fence_cost(filename: &str) -> usize {
        let map = read_map(filename);
        let mut visited = vec![vec![false; map[0].len()]; map.len()];
        let mut result = 0;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if visited[y][x] {
                    continue;
                }
                let area = get_area_of(&map, (x, y));
                for (x, y) in area.iter() {
                    visited[*y][*x] = true;
                }
                let sides = calculate_area_sides(&area, &map);

                result += sides * area.len();
            }
        }

        result
        // 942774
    }
}
