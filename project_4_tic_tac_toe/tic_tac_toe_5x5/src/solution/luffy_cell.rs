use tic_tac_toe_stencil::board::Cell;
use crate::solution::luffy_board::LuffyBoard;

pub struct LuffyCell {
    // Board
    pub board: LuffyBoard,

    // Index
    pub index: usize,
    pub point: (usize, usize),

    // Neighbors
    pub neighbours: (
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
    ),

    // Value of Cell
    pub value: Cell,

    // Scores for Cell
    pub streaks: (usize, usize)
}

impl LuffyCell {
    pub fn initialize_neighbors(&mut self) {

    }

    pub fn update_relations(&mut self) {

    }

    pub fn update_value(&mut self) {

    }

    pub fn notate(&self) -> char {
        match self.value {
            Cell::X => 'x',
            Cell::O => 'o',
            Cell::Wall => 'w',
            Cell::Empty => 'e'
        }
    }
}
