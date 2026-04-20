use tic_tac_toe_stencil::board::Cell;
use crate::solution::luffy_board::LuffyBoard;

pub struct LuffyCell {
    // Board
    pub board: LuffyBoard,

    // Index
    pub index: usize,
    pub point: (usize, usize),
    pub distance: f32,
    pub entropy: f32,

    // Value of Cell
    pub value: Cell,

    // Neighbors
    pub neighbours: (
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>
    ),

    // Friends
    pub friends_x: u64,
    pub friends_o: u64,

    // Streaks
    pub streaks_x: u64,
    pub streaks_o: u64
}

impl LuffyCell {
    pub fn initialize(&mut self) {

    }

    pub fn refresh(&mut self) {

    }

    pub fn update(&mut self, value: Value) {

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
