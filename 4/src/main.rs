use std::fs;
use array2d::Array2D;

const BOARDSIZE : usize = 5;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();
    let mut tmp : Vec<&str> = lines[0].split(',').collect();

    let bingo_numbers : Vec<u32> = tmp.clone().iter().map(|x| x.trim().parse().expect("panik")).collect();
    let mut boards : Vec<Array2D<u32>> = Vec::new();
    
    let mut i = 1;
    let mut row_iter = 0;
    let mut new_board = Array2D::filled_with(0, BOARDSIZE, BOARDSIZE);
    while i < lines.len() {
        // make new arr2d
        if lines[i].chars().count() == 0 {
            row_iter = 0;
        }
        else {
            tmp = lines[i].split_whitespace().collect();
            let row : Vec<u32> = tmp.clone().iter().map(|x| x.trim().parse().expect("panik")).collect();
            for index in 0..row.len() {
                new_board[(row_iter as usize, index as usize)] = row[index];
            }
            if row_iter == BOARDSIZE - 1 {
                boards.push(new_board.clone());
            }
            row_iter += 1;
        }
        i += 1;
    }

    for board in boards {
        print_arr2d(board);
        print!("\n");
    }
}

fn print_arr2d(arr: Array2D<u32>) {
    for i in 0..arr.num_rows() {
        for j in 0..arr.num_columns() {
            print!("{} ", arr[(i as usize, j as usize)]);
        }
        print!("\n");
    }
}