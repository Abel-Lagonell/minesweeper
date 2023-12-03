use crate::board::{Board, self};
use crate::cell::CellState;
use core::num;
use std::{io, fmt};

struct Options {
    first_zero: bool,
}

impl Options {
    fn new() -> Options {
        Options {
            first_zero: false,
        }
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "1. First Zero: {}", self.first_zero)
    }
}

pub struct Interface {
    running: bool,
    options: Options,
    turn: u16,
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            running : true,
            options : Options::new(),
            turn: 0,
        }
    }

    fn print_options(&self){
        println!("Type 'start' to begin");
        println!("Type 'options' to change options");
        println!("Type 'end' to quit");
    }

    pub fn run(&mut self){
        println!("Welcome to Minesweeper!");
        while self.running {
            self.print_options();
            match self.get_user_input().trim() {
                "s" => self.start_game(),
                "start" => self.start_game(),
                "o" => self.select_options(),
                "options" => self.select_options(),
                "e" => self.running = false,
                "end" => self.running = false,
                _ => println!("Invalid command"),
            }
        
        }
    }

    fn get_user_input(&self) -> String {
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input
    }

    fn select_options(&mut self) {
        println!("Toggle options");
        println!("0. Exit");
        println!("{}", self.options);
        println!("Type number to toggle");
        match self.get_user_input().trim().parse::<u8>().unwrap() {
            1 => self.options.first_zero = !self.options.first_zero,
            0 => (),
            _ => println!("Invalid option"),
        }
    }

    fn start_game(&mut self) {
        println!("Enter the size of the board");
        let size = self.get_user_input().trim().parse::<usize>().unwrap();
        println!("Enter the number of bombs");
        let bombs = self.get_user_input().trim().parse::<u8>().unwrap();
        let mut board = Board::new(bombs, size);
        board.print_current_board();
        self.turn(&mut board);
        board.populate_board();
        board.print_current_board();
        board.print_solved_board();
        println!("END OF GAME type end to quit")
    }

    fn turn(&mut self, board:&mut Board){
        self.turn += 1;

        println!("Enter the row");
        let row: usize = self.get_user_input().trim().parse::<usize>().unwrap();
        println!("Enter the column");
        let col: usize = self.get_user_input().trim().parse::<usize>().unwrap();
        
        println!("Enter the action");
        println!("1. Reveal");
        println!("2. Toggle Flag");
        match self.get_user_input().trim().parse::<u8>().unwrap() {
            1 =>
            {   
                if self.options.first_zero == false {self.reveal(row,col, board); println!("Revealing 1");}
                else {self.reveal_8(row,col,  board); println!("Revealing 8");}
            },
            2 => board[row][col].toggle_flag(),
            _ => println!("Invalid action"),
        }
    }

    fn reveal(&mut self, row: usize, col: usize, board: &mut Board){
        if self.turn == 1 {
            board[row][col].reveal();
            if board[row][col].get_bombs_nearby() != 0 {return}
        }
        if board[row][col].is_bomb() {println!("You lose"); self.running = false; return}
        
        let cell = &mut board[row][col];
        let num_bombs = cell.get_bombs_nearby();
        
        if 0 == num_bombs {
            self.reveal_8(row, col, board);
        }

    }

    fn reveal_8(&mut self, row: usize, col: usize, b:&mut Board) {
        
        match b[row][col].get_state() {
            CellState::Revealed => return,
            CellState::Flagged => return,
            _ => (),
        };

        self.reveal(row, col, b);

        if row > 0 {
            self.reveal(row-1, col, b);

            if col > 0 {self.reveal(row-1, col-1, b);}
            if col < b.get_size()-1 {self.reveal(row-1, col+1, b);}
        }
        if row < b.get_size()-1 {
            self.reveal(row+1, col, b);
            if col > 0 {self.reveal(row+1, col-1, b);}
            if col < b.get_size()-1 {self.reveal(row+1, col+1, b);}
        }
        if col > 0 {self.reveal(row, col-1, b);}
        if col < b.get_size()-1 {self.reveal(row, col+1, b);}
    }

    fn count_flags(&self, row: usize, col: usize, b: Board) -> u8{
        let mut count = 0;
        if row > 0 {
            if b[row-1][col].is_flagged() {count += 1;}
            if col > 0 {if b[row-1][col-1].is_flagged() {count += 1;}}
            if col < b.get_size()-1 {if b[row-1][col+1].is_flagged() {count += 1;}}
        }
        if row < b.get_size()-1 {
            if b[row+1][col].is_flagged() {count += 1;}
            if col > 0 {if b[row+1][col-1].is_flagged() {count += 1;}}
            if col < b.get_size()-1 {if b[row+1][col+1].is_flagged() {count += 1;}}
        }
        if col > 0 {if b[row][col-1].is_flagged() {count += 1;}}
        if col < b.get_size()-1 {if b[row][col+1].is_flagged() {count += 1;}}
        return count;
    }



}
