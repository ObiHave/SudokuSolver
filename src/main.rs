use std::thread;
use std::time::Duration;
fn main() {
    /*
    let correct = 
    [[5,3,4,6,7,8,9,1,2].to_vec(), 
     [6,7,2,1,9,5,3,4,8].to_vec(), 
     [1,9,8,3,4,2,5,6,7].to_vec(), 
     [8,5,9,7,6,1,4,2,3].to_vec(), 
     [4,2,6,8,5,3,7,9,1].to_vec(), 
     [7,1,3,9,2,4,8,5,6].to_vec(), 
     [9,6,1,5,3,7,2,8,4].to_vec(), 
     [2,8,7,4,1,9,6,3,5].to_vec(), 
     [3,4,5,2,8,6,1,7,8].to_vec()].to_vec();

     let input = 
    [[5,3,0,0,7,0,0,0,0].to_vec(), 
    [6,0,0,1,9,5,0,0,0].to_vec(), 
    [0,9,8,0,0,0,0,6,0].to_vec(), 
    [8,0,0,0,6,0,0,0,3].to_vec(), 
    [4,0,0,8,0,3,0,0,1].to_vec(), 
    [7,0,0,0,2,0,0,0,6].to_vec(), 
    [0,6,0,0,0,0,2,8,0].to_vec(), 
    [0,0,0,4,1,9,0,0,5].to_vec(), 
    [0,0,0,0,8,0,0,7,9].to_vec()].to_vec();

    let input = 
    [[8,0,0,0,0,0,0,0,0].to_vec(), 
    [0,0,3,6,0,0,0,0,0].to_vec(), 
    [0,7,0,0,9,0,2,0,0].to_vec(), 
    [0,5,0,0,0,7,0,0,0].to_vec(), 
    [0,0,0,0,4,5,7,0,0].to_vec(), 
    [0,0,0,1,0,0,0,3,0].to_vec(), 
    [0,0,1,0,0,0,0,6,8].to_vec(), 
    [0,0,8,5,0,0,0,1,0].to_vec(), 
    [0,9,0,0,0,0,4,0,0].to_vec()].to_vec();
    */
    let input = 
    [[7,0,0,0,0,0,0,8,0].to_vec(), 
    [3,0,0,2,0,6,0,0,4].to_vec(), 
    [5,0,0,0,0,9,0,2,0].to_vec(), 
    [0,0,0,0,8,0,0,5,0].to_vec(), 
    [0,0,2,3,0,4,0,6,0].to_vec(), 
    [0,0,9,0,0,0,0,0,1].to_vec(), 
    [8,0,6,0,0,1,3,0,2].to_vec(), 
    [0,1,0,0,4,0,7,0,0].to_vec(), 
    [0,4,0,0,0,8,0,0,5].to_vec()].to_vec();
/*
    let input = 
    [[0,0,5,3,0,0,0,0,0].to_vec(), 
    [8,0,0,0,0,0,0,2,0].to_vec(), 
    [0,7,0,0,1,0,5,0,0].to_vec(), 
    [4,0,0,0,0,5,3,0,0].to_vec(), 
    [0,1,0,0,7,0,0,0,6].to_vec(), 
    [0,0,3,2,0,0,0,8,0].to_vec(), 
    [0,6,0,5,0,0,0,0,9].to_vec(), 
    [0,0,4,0,0,0,0,3,0].to_vec(), 
    [0,0,0,0,0,9,7,0,0].to_vec()].to_vec();
  */
    let solved = sudoku_solver(&input);
    println!("Solved:");
    print(&solved);
   // assert_eq!(solved, correct);
}

fn sudoku_solver(sudoku: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut game = sudoku.clone();
    recurse(&mut game);
    game
}

fn recurse(mut v: &mut std::vec::Vec<std::vec::Vec<i32>>) -> bool {
    match find_zero(&v) {
        Some((r, c)) => {
            for i in 1..10 {
                if valid(&mut v, r, c, i) {
                    v[r as usize][c as usize] = i;
                    if recurse(v) {
                        return true;
                    }
                    v[r as usize][c as usize] = 0;
                }
            }
        },
        None => {
            return true;
        }
    };
    return false;
}
/*
fn get_guesses(r:i32, c:i32, board: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec!();
    let column: Vec<i32> = board.iter().map(|x| x[c as usize]).collect();
    let subgrid = get_subgrid(r, c, board);
    
    for guess in 1..10 {
       // println!("Col: {:?}, Row: {:?}, Subgrid: {:?}, Guess: {}", column, board[r as usize], subgrid, guess);
       // Check the row, column and subgrid if the item does not exist.
        if !board[r as usize].contains(&guess) && !column.contains(&guess) && !subgrid.contains(&guess) {
            result.push(guess);   
        } 
    }
    //println!("Col: {:?}, Row: {:?}, Subgrid: {:?}, Res: {:?}", column, board[r as usize], subgrid, result);
    return result
}
*/

fn valid(board: &mut std::vec::Vec<std::vec::Vec<i32>>, r: i32, c: i32, num: i32) -> bool {
    let mut bo = board.clone();
    bo[r as usize][c as usize] = num;
    // Check row for duplicates.
    //println!("Row: {:?}", bo[r as usize]);
    if contains_duplicates(&bo[r as usize]) {
        //println!("Found Duplicate.");
        return false
    }
    // Check column for duplicates.
    let column: Vec<i32> = bo.iter().map(|x| x[c as usize]).collect();
   // println!("Col: {:?}", column);
    if contains_duplicates(&column) {
        //println!("Found Duplicate.");
        return false
    }
    // Check subgrid for duplicates
    let subgrid = get_subgrid(r, c, &bo);
    //       println!("Subgrid: {:?}", subgrid);
    if contains_duplicates(&subgrid) {
    //    println!("Found Duplicate.");
        return false
    }
    return true
}

fn find_zero(v: &Vec<Vec<i32>>) -> Option<(i32, i32)> {
    for r in 0..9 {
        for c in 0..9 { 
            if v[r as usize][c as usize] == 0 {
                return Some((r, c));
            }
        }
    }
    return None
}

fn contains_duplicates(v: &Vec<i32>) -> bool{
    let mut track: Vec<i32> = [0;9].to_vec();
    for i in 0..9{
        if v[i as usize] > 0 {
           // println!("{:?}", v[i as usize]);
            track[(v[i as usize] - 1) as usize] =  track[(v[i as usize] - 1) as usize] + 1;
        }
    }
   // println!("Track: {:?}",track);
    let violations: Vec<i32> = track.into_iter().filter(|&x| x > 1).collect();
  //  println!("Violators:{:?}", violations);
    if violations.len() > 0 {
        return true
    }
    return false    
}

fn get_subgrid(r: i32, c:i32, board: &Vec<Vec<i32>>) -> Vec<i32> {
    let grid_col = c/3;
    let grid_row = r/3;
    let mut subgrid: Vec<i32> = vec!();
    for i in 3*grid_row..3*grid_row+3 {
        subgrid.push(board[i as usize][(grid_col*3) as usize]);
        subgrid.push(board[i as usize][(grid_col*3+1) as usize]);
        subgrid.push(board[i as usize][(grid_col*3+2) as usize]);
    }
    subgrid
}

fn print(v: &Vec<Vec<i32>>) {
    for i in 0..9 {
        println!("{:?}", v[i]);
    }
}