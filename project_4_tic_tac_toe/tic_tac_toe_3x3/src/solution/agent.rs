use std::collections::HashMap;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::layout::Layout;
use tic_tac_toe_stencil::player::Player;

use crate::layout::Layout3x3;

// Your solution solution.
pub struct SolutionAgent {}

// Pulls the board (or self if none) from the states hashmap
pub fn get_shape(states: &HashMap<String, Board>, board: &Board) -> String {
    todo!("smth");
}

// Checks if the states hashmap has a similar shape of self via transposition
pub fn has_shape(states: &HashMap<String, Board>, board: &Board) -> bool {
    todo!("smth");
}

// Converts the board into a string notation
pub fn hash_board(board: &Board) -> String {
    let two_d_map = board.get_cells();
    let mut hash_brown_fries = String::new();

    for vector in two_d_map {
        for cell in vector{
            match cell{
                Cell::X => hash_brown_fries.push_str("X"),
                Cell::O => hash_brown_fries.push_str("O"),
                Cell::Empty =>hash_brown_fries.push_str("_"),
               _=> panic!("Invalid cell!")
            }

        }
    }
    hash_brown_fries // and some ketchup and mickidies!!



}

// Rotates board clock-wise
pub fn rotate_roard(board: &Board) -> Board {
    let cells = board.get_cells();
    let mut rotated = Board::new(Layout3x3 {});
    for y in 0..cells.len() {
        for x in 0..cells.len() {
            let m = (y, x);
            let m_prime = match m {
                (0, 0) => (0, 2),
                (0, 2) => (2, 2),
                (2, 2) => (2, 0),
                (2, 0) => (0, 0),
                (0, 1) => (1, 2),
                (1, 2) => (2, 1),
                (2, 1) => (1, 0),
                (1, 0) => (0, 1),
                (1, 1) => (1, 1),
                _ => panic!("Invalid cell")
            };
            match cell_to_player(cells, m) {
                Some(player) => rotated.apply_move(m_prime, player),
                None => ()
            };
        }
    }
    rotated
}

// Flips board horizontally
pub fn flip_board(board: &Board) -> Board {
    let cells = board.get_cells();
    let mut flipped = Board::new(Layout3x3 {});
    for y in 0..cells.len() {
        for x in 0..cells.len() {
            let m = (y, x);
            let m_prime = (y, 2 - x);
            match cell_to_player(cells, m) {
                Some(player) => flipped.apply_move(m_prime, player),
                None => ()
            }
        }
    }
    flipped
}

// Turns Cell enum to Player enum at cell location
pub fn cell_to_player(cells: &Vec<Vec<Cell>>, m: (usize, usize)) -> Option<Player> {
    match cells[m.0][m.1] {
        Cell::X => Some(Player::X),
        Cell::O => Some(Player::O),
        _ => None
    }
}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        unimplemented!("Not yet implemented")
    }
}
