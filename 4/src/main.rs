use std::fs;
use array2d::Array2D;

const BOARDSIZE : usize = 5;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Something went wrong");

    let lines : Vec<&str> = file_contents.lines().collect();
    let mut tmp : Vec<&str> = lines[0].split(',').collect();

    let bingo_numbers : Vec<u32> = tmp.clone().iter().map(|x| x.trim().parse().expect("panik")).collect();
    let mut boards : Vec<Array2D<(u32, bool)>> = Vec::new();
    let mut winner_boards : Vec<(usize, u32)> = Vec::new();
    
    let mut i = 1;
    let mut row_iter = 0;
    let mut new_board = Array2D::filled_with((0, false), BOARDSIZE, BOARDSIZE);

    // populate boards vector from input
    while i < lines.len() {
        // make new arr2d
        if lines[i].chars().count() == 0 {
            row_iter = 0;
        }
        else {
            tmp = lines[i].split_whitespace().collect();
            let row : Vec<u32> = tmp.clone().iter().map(|x| x.trim().parse().expect("panik")).collect();
            for index in 0..row.len() {
                new_board[(row_iter as usize, index as usize)] = (row[index], false);
            }
            if row_iter == BOARDSIZE - 1 {
                boards.push(new_board.clone());
            }
            row_iter += 1;
        }
        i += 1;
    }

    'main: for number in bingo_numbers {
        println!("drawing... {}", number);
        for index in 0..boards.len() {
            let found = find_in_array2d(boards[index].clone(), number);
            
            if found.is_some() {
                let position = found.unwrap();
                boards[index][position].1 = true;
                let win = check_win(boards[index].clone(), position);
                if win {
                    let score = calc_board_score(boards[index].clone(), number);
                    if check_unique_winner(winner_boards.clone(), (index+1, score)) {
                        winner_boards.push((index+1, score));
                        println!("BOARD {} WON!", index + 1);
                        println!("SCORE = {}", score);
                    }
                }
            }
        }
    }

    println!("LAST BOARD WON WITH SCORE {}", winner_boards[winner_boards.len()-1].1);
}

fn check_unique_winner(winners: Vec<(usize, u32)>, entry: (usize, u32)) -> bool {
    for winner in winners {
        if winner.0 == entry.0 {
            return false;
        }
    }

    true
}

// not needed anymore but left as POC
fn print_arr2d(arr: Array2D<(u32, bool)>) {
    for i in 0..arr.num_rows() {
        for j in 0..arr.num_columns() {
            let tup : (u32, bool) = arr[(i as usize, j as usize)];
            print!("{} -{} ", tup.0, tup.1);
        }
        print!("\n");
    }
}

fn find_in_array2d(arr: Array2D<(u32, bool)>, num_to_find: u32) -> Option<(usize, usize)> {
    for i in 0..arr.num_rows() {
        for j in 0..arr.num_columns() {
            if arr[(i, j)].0 == num_to_find {
                return Some((i, j))
            }
        }
    }

    None
}

fn check_win(arr: Array2D<(u32, bool)>, positon: (usize, usize)) -> bool {
    let mut col_win = true;
    let mut row_win = true;

    // check column
    for i in 0..arr.num_rows() {
        if arr[(i, positon.1)].1 == false {
            col_win = false;
            break;
        } 
    }

    // check column
    for i in 0..arr.num_rows() {
        if arr[(positon.0, i)].1 == false {
            row_win = false;
            break;
        } 
    }

    col_win || row_win
}

fn calc_board_score(arr: Array2D<(u32, bool)>, number: u32) -> u32 {
    let mut sum_numarked = 0;
    for i in 0..arr.num_rows() {
        for j in 0..arr.num_columns() {
            if arr[(i, j)].1 == false {
                sum_numarked += arr[(i, j)].0;
            }
        }
    }

    sum_numarked * number
}