use std::ops::{Index, IndexMut};
use super::cell::{Cell, CellState};


pub struct Board {
    board: Vec<Vec<Cell>>,
    bombs: u8,
    size: usize,
}

impl Board {
    pub fn new(bombs: u8, size: usize) -> Board {
        Board {
            board: vec![vec![Cell::new(); size]; size],
            bombs: bombs,
            size: size,
        }
    }

    pub fn get_size(&self) -> usize{
        self.size
    }

    pub fn get_board(&self) -> &Vec<Vec<Cell>> {
        &self.board
    }

    // Mutable getter
    pub fn get_board_mut(&mut self) -> &mut Vec<Vec<Cell>> {
        &mut self.board
    }

    pub fn populate_board(&mut self) {
        if self.bombs > (self.size as u8 * self.size as u8 - 1) {println!("ERROR"); return}
        let mut count = 0;
        println!("Populating board");
        for _ in 0..self.bombs {
            let mut row: usize = rand::random::<usize>() % self.size;
            let mut col: usize = rand::random::<usize>() % self.size;
            while !self.board[row][col].set_bomb(){
                row = rand::random::<usize>() % self.size;
                col = rand::random::<usize>() % self.size;
                count += 1;
                if count == self.size*self.size {println!("ERROR"); return}
            }
            self.update_nearby(row, col);
        }
    }

    pub fn print_solved_board(&self) {
        for row in &self.board {
            for cell in row {
                if cell.is_bomb() {
                    print!("* ");
                } else{
                    print!("{:#?} ", cell.get_bombs_nearby());
                }
            }
            println!("");
        }
    }
    
    pub fn print_current_board(&self) {
        let board = &self.board;
        for row in board {
            for cell in row {
                match cell.get_state() {
                    CellState::Flagged => print!("F "),
                    CellState::Revealed => {
                        if cell.is_bomb() { print!("! ");}
                        else {print!("{} ", cell.get_bombs_nearby());}
                    },
                    CellState::Hidden => print!("X "),
                }

            }
            println!("");
        }
        println!("");
    }

    fn update_nearby(&mut self, row: usize, col: usize){
        let board = &mut self.board;
        if row > 0 {
            board[row-1][col].increment_bombs_nearby();
            if col > 0 {board[row-1][col-1].increment_bombs_nearby();}
            if col < board.len()-1 {board[row-1][col+1].increment_bombs_nearby();}
        }
        if row < board.len()-1 {
            board[row+1][col].increment_bombs_nearby();
            if col > 0 {board[row+1][col-1].increment_bombs_nearby();}
            if col < board.len()-1 {board[row+1][col+1].increment_bombs_nearby();}
        }
        if col > 0 {board[row][col-1].increment_bombs_nearby();}
        if col < board.len()-1 {board[row][col+1].increment_bombs_nearby();}
    }
    

    
}   

//Easier implementation of indexing such that board[row][col] automatically returns the cell at that position

impl Index<usize> for Board {
    type Output = Vec<Cell>;

    fn index(&self, index: usize) -> &Vec<Cell> {
        &self.board[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Cell> {
        &mut self.board[index]
    }
}