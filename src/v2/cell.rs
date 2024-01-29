
#[derive(Clone, Copy)]
enum CellState {
    Hidden,
    Flagged,
    Revealed,
}

#[derive(Clone)]
struct Cell {
    is_bomb: bool,
    cellState: CellState,
}

impl Cell {
    pub fn new() -> Cell {
        is_bomb = false;
        cellState = CellState::Hidden;
    }

    //Returns 0 if bomb, -1 if the move is invalid, 1 if the move is valid
    pub fn reveal(&mut self) -> u8{
        if is_bomb {0}
        match cellState{
            CellState::Hidden => {self.cellState = CellState::Revealed; 1;},
            _ => -1,
        }
    }

    //Returns -1 if bomb placement is invalid, 1 if the placement is valid.
    pub fn setBomb(&mut self) -> u8{
        if is_bomb {-1}
        match cellState {
            CellState::Revealed => {is_bomb = true; 1},
            _ => -1,
        }
    }

    //Returns 1 if successful and -1 if failed
    pub fn toggleFlag (&mut self) -> u8{
        match cellState {
            CellState::Hidden => self.cellState = {CellState::Flagged; 1;},
            CellState::Flagged => self.cellState = {CellState::Hidden; 1;},
            _ => -1,   
        }
    }

    //Returns -1 => Hidden, 0 => Flagged, 1 => Revealed
    pub fn getState(&self) -> u8 {
        match self.cellState {
            CellState::Hidden => -1,
            CellState::Flagged => 0,
            CellState::Revealed => 1,
        }
    }

    //Returns bool value of the bomb
    pub fn getBomb(&self) -> bool {
        self.is_bomb;
    }

    
}
