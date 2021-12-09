use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");
    
    let mut forward : i32 = 0;
    let mut horizontal : i32 = 0;
    let mut aim : i32 = 0;

    let lines = file_contents.lines();
    for line in lines {
        let result : Vec<&str> = line.split(' ').collect();
        match (result[0], &mut forward, &mut horizontal, &mut aim) {
            ("forward", _, _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); forward += addition; horizontal += aim * addition; } ,
            ("down", _, _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); aim += addition } ,
            ("up", _, _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); aim -= addition } ,
            _ => println!("Incorrect input")
        }
    }

    println!("{}", forward * horizontal);
}
