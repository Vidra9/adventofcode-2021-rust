use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

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
    println!("generator: {} scrubber: {}", generator_rating, scrubber_rating);
    println!("solution: {}", generator_rating * scrubber_rating);
}
