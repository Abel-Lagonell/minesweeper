// TODO: Make the board a struct with implementation
#[derive(Clone)]
struct Cell {
    is_bomb: bool,
    is_first: bool,
    is_revealed: bool,
    //is_flagged: bool,
    bombs_nearby: u8,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            is_bomb: false,
            is_first: false,
            is_revealed: false,
            //is_flagged: false,
            bombs_nearby: 0,
        }
    }
}

fn main() {
    let bombs: u8 = 3;
    let size: usize = 4;
    

    let mut board: Vec<Vec<Cell>> = vec![vec![Cell::new(); size]; size];
    board[1][2].is_first = true;
    reveal(&mut board, 1, 2);

    populate_board(size, &mut board, bombs);

    print_current_board(&board);
    println!("");
    print_solved_board(&board);

}


fn populate_board(size: usize, board: &mut Vec<Vec<Cell>>, bombs: u8) {
    for _ in 0..bombs {
        let mut x: usize = rand::random::<usize>() % size;
        let mut y: usize = rand::random::<usize>() % size;
        while board[x][y].is_bomb || board[x][y].is_first {
            x = rand::random::<usize>() % size;
            y = rand::random::<usize>() % size;
        }
        board[x][y].is_bomb = true;
        update_nearby(board, x, y)
    }
}

fn update_nearby(board: &mut Vec<Vec<Cell>>, x: usize, y: usize){
    if x > 0 {
        board[x-1][y].bombs_nearby += 1;
        if y > 0 {board[x-1][y-1].bombs_nearby += 1;}
        if y < board.len()-1 {board[x-1][y+1].bombs_nearby += 1;}
    }
    if x < board.len()-1 {
        board[x+1][y].bombs_nearby += 1;
        if y > 0 {board[x+1][y-1].bombs_nearby += 1;}
        if y < board.len()-1 {board[x+1][y+1].bombs_nearby += 1;}
    }
    if y > 0 {board[x][y-1].bombs_nearby += 1;}
    if y < board.len()-1 {board[x][y+1].bombs_nearby += 1;}
}

fn print_solved_board(board: &Vec<Vec<Cell>>) {
    for row in board {
        for cell in row {
            if cell.is_bomb {
                print!("* ");
            } else{
                print!("{} ", cell.bombs_nearby);
            }
        }
        println!("");
    }
}

fn print_current_board(board: &Vec<Vec<Cell>>) {
    for row in board {
        for cell in row {
            if cell.is_revealed {
                if cell.is_bomb {
                    print!("* ");
                } else{
                    print!("{} ", cell.bombs_nearby);
                }
            } else {
                print!(". ");
            }
        }
        println!("");
    }
}

fn reveal(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) { // TODO: make this reveal all adjacent cells if bombs_nearby == 0, Currently only reveals the whole board
    if board[x][y].is_revealed {return;}
    board[x][y].is_revealed = true;
    if board[x][y].bombs_nearby != 0 {return;}
    if board[x][y].bombs_nearby == 0 {
        if x > 0 {
            reveal(board, x-1, y);
            if y > 0 {reveal(board, x-1, y-1);}
            if y < board.len()-1 {reveal(board, x-1, y+1);}
        }
        if x < board.len()-1 {
            reveal(board, x+1, y);
            if y > 0 {reveal(board, x+1, y-1);}
            if y < board.len()-1 {reveal(board, x+1, y+1);}
        }
        if y > 0 {reveal(board, x, y-1);}
        if y < board.len()-1 {reveal(board, x, y+1);}
    }
}