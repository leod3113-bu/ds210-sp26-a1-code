use tic_tac_toe_stencil::board::Cell;
use crate::solution::luffy_board::LuffyBoard;

pub struct LuffyCell {
    // Board
    board: LuffyBoard,

    // Index
    index: usize,
    point: (usize, usize),

    // Neighbors
    neighbours: (
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
        Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>, Option<Box<LuffyCell>>,
    ),

    // Value of Cell
    value: Cell,

    // Scores for Cell
    streaks: (usize, usize)
}

impl LuffyCell {
    fn initialize_neighbors(&mut self) {

    }

    fn update_relations(&mut self) {

    }

    fn update_value(&mut self) {

    }

    fn notate(&self) -> char {
        match self.value {
            Cell::X => 'x',
            Cell::O => 'o',
            Cell::Wall => 'w',
            Cell::Empty => 'e'
        }
    }
}
