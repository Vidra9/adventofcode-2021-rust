use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");
    
    let mut forward : i32 = 0;
    let mut horizontal : i32 = 0;

    let lines = file_contents.lines();
    for line in lines {
        let result : Vec<&str> = line.split(' ').collect();
        match (result[0], &mut forward, &mut horizontal) {
            ("forward", _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); forward += addition } ,
            ("down", _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); horizontal += addition } ,
            ("up", _, _) => { let addition : i32 = result[1].trim().parse().expect("Not a number"); horizontal -= addition } ,
            _ => println!("Incorrect input")
        }
    }

    println!("{}", forward * horizontal);
}
