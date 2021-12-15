use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

    solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
    let mut occupied : Vec<(u32, u32, u32)> = Vec::new();

    let mut line_num = 0;
    for line in lines {
        line_num += 1;
        let first_split : Vec<&str> = line.split_whitespace().collect();
        let start_position : Vec<&str> = first_split[0].split(',').collect();
        let start_position : Vec<u32> = start_position.iter().map(|x| x.trim().parse().expect("panik")).collect(); // x, y
        let start_position = (start_position[0], start_position[1]);
        let end_position : Vec<&str> = first_split[2].split(',').collect();
        let end_position : Vec<u32> = end_position.iter().map(|x| x.trim().parse().expect("panik")).collect(); // x, y
        let end_position = (end_position[0], end_position[1]);

        let mut coord = 0; // x = 0, y = 1
        let mut loop_range = if start_position.0 == end_position.0 {(start_position.1, end_position.1)} 
            else if start_position.1 == end_position.1 {coord = 1; (start_position.0, end_position.0)} else {continue}; // x, x || y, y
        // (8, 3) -> (2, 3)

        if loop_range.0 > loop_range.1 {
            let tmp = loop_range.1;
            loop_range.1 = loop_range.0;
            loop_range.0 = tmp;
        }

        print!("start: {},{} -- ", start_position.0, start_position.1);
        print!("end: {},{} -- ", end_position.0, end_position.1);
        print!("loop range: {},{}", loop_range.0, loop_range.1);
        println!("line num: {}", line_num);

        for i in loop_range.0..=loop_range.1 {
            let pos = if coord == 0 {(start_position.0, i)} else {(i, start_position.1)};
            let index = find(occupied.clone(), pos);
            if index.is_some() {
                occupied[index.unwrap()].2 += 1;
                // print!("inc ({},{},{}) ", occupied[index.unwrap()].0, occupied[index.unwrap()].1, occupied[index.unwrap()].2);
            }
            else {
                occupied.push((pos.0, pos.1, 1));
                // print!("push ({},{}) ", pos.0, pos.1);
            }
        }
        // print!("\n");
    }
    
    let mut result = 0;
    for bruh in occupied {
        if bruh.2 > 1 {
            result += 1;
        }
    }

    println!("part 1: {}", result);
}

fn find(vec : Vec<(u32, u32, u32)>, pos_to_find : (u32, u32)) -> Option<usize> {
    for index in 0..vec.len() {
        if vec[index].0 == pos_to_find.0 && vec[index].1 == pos_to_find.1 {
            return Some(index)
        }
    }
    None
}

fn solve_part2(lines : Vec<&str>) {

}