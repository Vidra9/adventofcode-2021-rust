use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();
    
    solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
    let lines_len = lines.len() as u32;
    let mut counter_vector : Vec<u32> = Vec::new();
    init_vector_with_num(&mut counter_vector, 0, lines[0].chars().count() as u32);

    for line in lines {
        for i in 0..line.chars().count() {
            let character : char = line.chars().nth(i).unwrap();
            if character.to_digit(10) == Some(1) {
                counter_vector[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let base : i32 = 2;
    let x = counter_vector.len() as u32;
    for index in 0..x {
        let item = counter_vector[index as usize];
        let power = x - index - 1;
        if item >= lines_len/2 {
            gamma += base.pow(power);
        }
        else {
            epsilon += base.pow(power);
        }
    }

    println!("part 1 : {}", gamma * epsilon);
}

fn solve_part2(lines : Vec<&str>) {
    let mut generator_vec : Vec<&str> = lines;
    let mut scrubber_vec : Vec<&str> = generator_vec.clone();

    {
        let mut generator_index = 0;
        
        while generator_vec.len() > 1 {
            let mut one_counter = 0;
            for item in generator_vec.clone() {
                let character : char = item.chars().nth(generator_index).unwrap();
                if character.to_digit(10) == Some(1) {
                    one_counter += 1;
                }
            }
            let digit = if one_counter >= generator_vec.len() - one_counter {'1'} else {'0'};
            
            generator_vec.retain(|&s| s.chars().nth(generator_index).unwrap() == digit);
            generator_index += 1;
        }

    }

    {
        let mut scrubber_index = 0;
        
        while scrubber_vec.len() > 1 {
            let mut one_counter = 0;
            for item in scrubber_vec.clone() {
                let character : char = item.chars().nth(scrubber_index).unwrap();
                if character.to_digit(10) == Some(1) {
                    one_counter += 1;
                }
            }
            let digit = if one_counter >= scrubber_vec.len() - one_counter {'0'} else {'1'};

            scrubber_vec.retain(|&s| s.chars().nth(scrubber_index).unwrap() == digit);
            scrubber_index += 1;
        }

    }
    let generator_rating = isize::from_str_radix(generator_vec[0], 2).unwrap();
    let scrubber_rating = isize::from_str_radix(scrubber_vec[0], 2).unwrap();
    println!("part 2: {}", generator_rating * scrubber_rating);
}

fn init_vector_with_num(vector: &mut Vec<u32>, num: u32, size: u32) {
    for _ in 0..size {
        vector.push(num);
    }
}