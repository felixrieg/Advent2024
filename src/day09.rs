pub mod day09 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn read_string(filename: &str) -> String {
        let file = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(file);

        let mut buffer = String::new();
        let line = reader.read_line(&mut buffer);
        match line {
            Ok(0) | Err(_) => panic!("File is empty"),
            Ok(_) => {}
        }
        buffer
    }

    pub fn calculate_checksum_from_file(filename: &str) -> usize {
        let input = read_string(filename);

        let mut low_id = 0;
        let mut high_id = (input.len() / 2) - 1;

        let mut start_pointer = 0;
        let mut end_pointer = input.len() - 1;

        end_pointer = if end_pointer % 2 == 0 {
            end_pointer
        } else {
            end_pointer - 1
        };

        let mut remaining_high = input
            .chars()
            .nth(end_pointer)
            .unwrap()
            .to_digit(10)
            .unwrap();

        let mut index = 0;
        let mut checksum = 0;

        while start_pointer < end_pointer {
            let block_size = input
                .chars()
                .nth(start_pointer)
                .unwrap()
                .to_digit(10)
                .unwrap();

            if start_pointer % 2 == 0 {
                // data block
                for _ in 0..block_size {
                    checksum += index * low_id;
                    index += 1;
                }
                low_id += 1;
                start_pointer += 1;
            } else {
                // free block
                for _ in 0..block_size {
                    if remaining_high <= 0 {
                        end_pointer -= 2;
                        high_id -= 1;
                        if start_pointer >= end_pointer {
                            break;
                        }
                        remaining_high = input
                            .chars()
                            .nth(end_pointer)
                            .unwrap()
                            .to_digit(10)
                            .unwrap();
                    }
                    remaining_high -= 1;
                    checksum += index * high_id;
                    index += 1;
                }
                start_pointer += 1;
            }
        }
        for _ in 0..remaining_high {
            checksum += index * high_id;
            index += 1;
        }

        checksum as usize
    }

    pub fn calculate_checksum_refragged_from_file(filename: &str) -> usize {
        let input = read_string(filename);

        let length = input.len() - 1;

        let mut block_sizes = vec![0; length];
        let mut block_ids: Vec<isize> = vec![-1; length];

        for i in 0..length {
            block_sizes[i] = input.chars().nth(i).unwrap().to_digit(10).unwrap();
            if i % 2 == 0 {
                block_ids[i] = (i / 2) as isize;
            }
        }

        for mut i in (0..length).rev() {
            if block_ids[i] == -1 {
                continue;
            }
            let needed_size = block_sizes[i];
            for j in 0..i {
                if block_ids[j] == -1 && block_sizes[j] > needed_size {
                    // found block
                    let remaining_size = block_sizes[j] - needed_size;

                    block_ids[j] = block_ids[i];
                    block_ids[i] = -1;
                    block_sizes[j] = needed_size;
                    block_sizes.insert(j + 1, remaining_size);
                    block_ids.insert(j + 1, -1);
                    i += 1;

                    break;
                } else if block_ids[j] == -1 && block_sizes[j] >= needed_size {
                    // found fitting block
                    block_ids[j] = block_ids[i];
                    block_ids[i] = -1;
                    break;
                }
            }
        }

        let mut index = 0;
        let mut checksum = 0;

        for i in 0..block_sizes.len() {
            let block_size = block_sizes[i];
            let id = match block_ids[i] {
                -1 => 0,
                i => i,
            };

            for _ in 0..block_size {
                checksum += index * id;
                index += 1;
            }
        }

        checksum as usize
    }
}
