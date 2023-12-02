use crate::board::Board;
use crate::cell::CellState;
use std::{io, fmt};
use do_while::do_while;

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
    board: Board,
    running: bool,
    options: Options,
}

impl Interface {
    pub fn new(_board:Board) -> Interface {
        Interface {
            board : _board,
            running : true,
            options : Options::new(),
        }
    }

    pub fn run(&mut self){
        println!("Welcome to Minesweeper!");
        println!("Type 'start' to begin");
        println!("Type 'end' to quit");
        while self.running {
            match self.get_user_input().trim() {
                "start" => self.start_game(),
                "end" => self.running = false,
                "options" => self.select_options(),
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
        println!("{}", self.options);
        println!("Type number to toggle");
        let input = self.get_user_input().trim().parse::<u8>().unwrap();
        match input {
            1 => self.options.first_zero = !self.options.first_zero,
            _ => println!("Invalid option"),
        }
    }

    fn start_game(&mut self) {
        println!("Enter the size of the board");
        let size = self.get_user_input().trim().parse::<usize>().unwrap();
        println!("Enter the number of bombs");
        let bombs = self.get_user_input().trim().parse::<u8>().unwrap();
        self.board = Board::new(bombs, size);
        self.board.print_current_board();
        self.turn();
        self.board.populate_board();
        self.board.print_current_board();
        self.board.print_solved_board();
        println!("END OF GAME type end to quit")
    }

    fn turn(&mut self){
        let mut col: usize;
        let mut row: usize;
        
        do_while! {
            do {
                println!("Enter valid row larger than 0");
                row = self.get_user_input().trim().parse::<usize>().unwrap()-1;
            }while row > self.board.get_size();
        };
        do_while!{
            do {
                println!("Enter valid col larger than 0");
                col = self.get_user_input().trim().parse::<usize>().unwrap()-1;
            }while col > self.board.get_size();
        };
        
        println!("Enter the action");
        println!("1. Reveal");
        println!("2. Toggle Flag");
        let action = self.get_user_input().trim().parse::<u8>().unwrap();
        match action {
            1 =>
            {
                if self.options.first_zero == false {self.board[row][col].reveal()}
                else {self.reveal_8(row,col)}
            },
            2 => self.board[row][col].toggle_flag(),
            _ => println!("Invalid action"),
        }
    }

    fn reveal_8(&mut self, x: usize, y: usize) {
        let b = self.board.get_board_mut();
        match b[x][y].get_state() {
            CellState::Revealed => return,
            CellState::Flagged => return,
            _ => (),
        };
        b[x][y].reveal();
        
        if x > 0 {
            b[x-1][y].reveal();
            if y > 0 {b[x-1][y-1].reveal();}
            if y < b.len()-1 {b[x-1][y+1].reveal();}
        }
        if x < b.len()-1 {
            b[x+1][y].reveal();
            if y > 0 {b[x+1][y-1].reveal();}
            if y < b.len()-1 {b[x+1][y+1].reveal();}
        }
        if y > 0 {b[x][y-1].reveal();}
        if y < b.len()-1 {b[x][y+1].reveal();}
    }

    fn reveal (&mut self, x: usize, y: usize){
        let b = self.board.get_board_mut();
        let cell = &mut b[x][y];
        let num_bombs = cell.get_bombs_nearby();
        let num_revealed = self.count_revealed(x, y);
        let num_flags = self.count_flags(x, y);
        
        if num_flags == num_bombs || 0 == num_bombs {
            self.reveal_8(x, y);
        } else if 8-num_revealed == num_bombs{
            self.mark_rest(x, y);
        }
    }

    fn count_flags(&self, x: usize, y: usize) -> u8{
        let b = self.board.get_board();
        let mut count = 0;
        if x > 0 {
            if b[x-1][y].is_flagged() {count += 1;}
            if y > 0 {if b[x-1][y-1].is_flagged() {count += 1;}}
            if y < b.len()-1 {if b[x-1][y+1].is_flagged() {count += 1;}}
        }
        if x < b.len()-1 {
            if b[x+1][y].is_flagged() {count += 1;}
            if y > 0 {if b[x+1][y-1].is_flagged() {count += 1;}}
            if y < b.len()-1 {if b[x+1][y+1].is_flagged() {count += 1;}}
        }
        if y > 0 {if b[x][y-1].is_flagged() {count += 1;}}
        if y < b.len()-1 {if b[x][y+1].is_flagged() {count += 1;}}
        return count;
    }

    fn count_revealed(&self, x: usize, y: usize) -> u8{
        let b = self.board.get_board();
        let mut count = 0;
        if x > 0 {
            if b[x-1][y].is_revealed() {count += 1;}
            if y > 0 {if b[x-1][y-1].is_revealed() {count += 1;}}
            if y < b.len()-1 {if b[x-1][y+1].is_revealed() {count += 1;}}
        }
        if x < b.len()-1 {
            if b[x+1][y].is_revealed() {count += 1;}
            if y > 0 {if b[x+1][y-1].is_revealed() {count += 1;}}
            if y < b.len()-1 {if b[x+1][y+1].is_revealed() {count += 1;}}
        }
        if y > 0 {if b[x][y-1].is_revealed() {count += 1;}}
        if y < b.len()-1 {if b[x][y+1].is_revealed() {count += 1;}}
        return count;
    }

    fn mark_rest (&mut self,x: usize, y: usize) {
        let b = self.board.get_board_mut();
        if x > 0 {
            b[x-1][y].flag();
            if y > 0 {b[x-1][y-1].flag();}
            if y < b.len()-1 {b[x-1][y+1].flag();}
        }
        if x < b.len()-1 {
            b[x+1][y].flag();
            if y > 0 {b[x+1][y-1].flag();}
            if y < b.len()-1 {b[x+1][y+1].flag();}
        }
        if y > 0 {b[x][y-1].flag();}
        if y < b.len()-1 {b[x][y+1].flag();}
    }
}
