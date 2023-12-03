#[derive(Clone, Copy)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
pub struct Cell {
    is_bomb: bool,
    cell_state: CellState,
    bombs_nearby: u8, // Keeping it as small as possible so four bits is enough
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            is_bomb: false,
            cell_state: CellState::Hidden,
            bombs_nearby: 0,
        } 
    }

    pub fn increment_bombs_nearby(&mut self) {
        self.bombs_nearby += 1;
    }

    pub fn reveal(&mut self) {
        match self.cell_state {
            CellState::Hidden => self.cell_state = CellState::Revealed,
            _ => (),
        }
    }

    //Returns if the placement of bomb is successful
    pub fn set_bomb(&mut self) -> bool{
        if self.is_bomb {return false;}
        match self.cell_state {
            CellState::Hidden => {self.is_bomb = true;  true},
            _ => return false,
            
        }
    }
    
    pub fn toggle_flag(&mut self) {
        self.cell_state = match self.cell_state {
            CellState::Hidden => CellState::Flagged,
            CellState::Flagged => CellState::Hidden,
            CellState::Revealed => CellState::Revealed,
        }
    }
    
    pub fn flag(&mut self) {
        match self.cell_state {
            CellState::Hidden => self.cell_state = CellState::Flagged,
            _ => (),
        }
        
    }

    pub fn is_bomb(&self) -> bool {
        self.is_bomb
    }

    pub fn get_state(&self) -> CellState {
        return self.cell_state;
    }

    pub fn get_bombs_nearby(&self) -> u8 {
        self.bombs_nearby
    }

    pub fn is_flagged(&self) -> bool {
        match self.cell_state {
            CellState::Flagged => true,
            _ => false,
        }
    }

    pub fn is_revealed(&self) -> bool {
        match self.cell_state {
            CellState::Revealed => true,
            _ => false,
        }
    }
    
}