use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();
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

    println!("gamma : {} epsilon : {}", gamma, epsilon);
    println!("result : {}", gamma * epsilon);
}

fn init_vector_with_num(vector: &mut Vec<u32>, num: u32, size: u32) {
    for _ in 0..size {
        vector.push(num);
    }
}