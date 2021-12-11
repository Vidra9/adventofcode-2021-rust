use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();

    solve_part1(lines.clone());
    solve_part2(lines.clone());
}

fn solve_part1(lines : Vec<&str>) {
    let mut forward : i32 = 0;
    let mut horizontal : i32 = 0;
    for line in lines {
        let result : Vec<&str> = line.split(' ').collect();
        match (result[0], &mut forward, &mut horizontal) {
            ("forward", _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); forward += addition } ,
            ("down", _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); horizontal += addition } ,
            ("up", _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); horizontal -= addition } ,
            _ => println!("Incorrect input")
        }
    }

    println!("part 1: {}", forward * horizontal);
}

fn solve_part2(lines : Vec<&str>) {
    
    let mut forward : i32 = 0;
    let mut horizontal : i32 = 0;
    let mut aim : i32 = 0;

    for line in lines {
        let result : Vec<&str> = line.split(' ').collect();
        match (result[0], &mut forward, &mut horizontal, &mut aim) {
            ("forward", _, _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); forward += addition; horizontal += aim * addition; } ,
            ("down", _, _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); aim += addition } ,
            ("up", _, _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); aim -= addition } ,
            _ => println!("Incorrect input")
        }
    }

    println!("part 2: {}", forward * horizontal);
}
