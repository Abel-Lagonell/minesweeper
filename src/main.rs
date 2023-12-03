
mod cell;
mod board;
mod interface;
use crate::interface::Interface;

fn main() {

    let mut ui = Interface::new();
    ui.run();
    
}

/*fn reveal_always(x: usize, y: usize, b: &mut Vec<Vec<Cell>>) { 
    if b[x][y].is_revealed() {return;}
    b[x][y].reveal();
    
    if b[x][y].get_bombs_nearby() == 0{
        if x > 0 {
            reveal_always(x-1, y, b);
            if y > 0 {reveal_always(x-1, y-1,b);}
            if y < b.len()-1 {reveal_always(x-1, y+1, b);}
        }
        if x < b.len()-1 {
            reveal_always(x+1, y, b);
            if y > 0 {reveal_always(x+1, y-1, b);}
            if y < b.len()-1 {reveal_always(x+1, y+1, b);}
        }
        if y > 0 {reveal_always(x, y-1, b);}
        if y < b.len()-1 {reveal_always(x, y+1, b);}
    }
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


fn flag_easy(x: usize, y: usize, b: &mut Vec<Vec<Cell>>) {
    let cell = &mut b[x][y];

    if cell.is_revealed(){
        let bombs = cell.get_bombs_nearby();
        let cells = count_valid_cells(x, y, b);
        let revealed_cells = count_revealed(x, y, b);
        let flagged_cells = count_flags(x, y, b);
        if flagged_cells == bombs {reveal_8(x, y, b)}
        if bombs == cells-revealed_cells {
            mark_rest(x, y, b)
        }
    } 
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

fn count_valid_cells(x: usize, y: usize, b: &Vec<Vec<Cell>>) -> u8{
    //Counts the number of cells that are not null
    let mut count:u8 = 0;
    if x > 0 {
        count += 1;
        if y > 0 {count += 1;}
        if y < b.len()-1 {count += 1;}
    }
    if x < b.len()-1 {
        count += 1;
        if y > 0 {count += 1;}
        if y < b.len()-1 {count += 1;}
    }
    if y > 0 {count += 1;}
    if y < b.len()-1 {count += 1;}
    return count;
}*/