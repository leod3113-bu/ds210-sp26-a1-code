use std::cmp;
use std::collections::{HashMap};

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::layout::Layout;
use tic_tac_toe_stencil::player::Player;

use crate::layout::Layout3x3;

// Your solution solution.
pub struct SolutionAgent {}

// Checks if the states hashmap has a similar shape of self via transposition
pub fn get_eval<'a>(evals: &'a mut HashMap<String, i32>, board: &Board) -> Option<&'a i32> {
    let mut mut_board = rotate_board(board);
    for _ in 0..4 {
        let hash = &hash_board(&mut_board);
        if evals.contains_key(hash) { return evals.get(hash); } // Rotating 360 degrees and 'snapshotting' and checking if composition already exists
        mut_board = rotate_board(&mut_board);
    }

    mut_board= flip_board(&mut_board);
    for _ in 0..4 {
        let hash = &hash_board(&mut_board);
        if evals.contains_key(hash) { return evals.get(hash); } // We can do the exact same steps as before
        mut_board = rotate_board(&mut_board);
    }
    None
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
pub fn rotate_board(board: &Board) -> Board {
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

// We are assuming we are maximizing for X
pub fn minimax(evals: &mut HashMap<String, i32>, board: &mut Board, maximizing: bool, depth: u64) -> i32 {
    if let Some(eval) = get_eval(evals, board) {
        return *eval;
    }

    let hash = hash_board(board);

    if board.game_over() {
        let score = board.score();

        if score == 0 {
            evals.insert(hash, 0);
            return 0;
        }

        let eval = if score > 0 { 10 } else { -10 };
        evals.insert(hash, eval);
        return eval;
    }

    if depth == 0 {
        return 0;
    }

    if maximizing {
        let mut max_eval = -1000;
        for player_move in board.moves() {
            board.apply_move(player_move, Player::X);
            let eval = minimax(evals, board, false, depth - 1);
            max_eval = cmp::max(max_eval, eval);
            board.undo_move(player_move, Player::X);
        }
        evals.insert(hash, max_eval);
        return max_eval;
    }
    else {
        let mut min_eval = 1000;
        for player_move in board.moves() {
            board.apply_move(player_move, Player::O);
            let eval = minimax(evals, board, true, depth - 1);
            min_eval = cmp::min(min_eval, eval);
            board.undo_move(player_move, Player::O);
        }
        evals.insert(hash, min_eval);
        return min_eval;
    }
}

pub fn find_best_move(board: &mut Board, player: Player, depth: u64) -> Option<(i32, usize, usize)> {
    match player {
        Player::X => {
            let mut max_eval = -1000;
            let mut best_move = None;
            for player_move in board.moves() {
                board.apply_move(player_move, Player::X);
                let eval = minimax(&mut HashMap::new(), board, false, depth);
                if eval > max_eval {
                    max_eval = eval;
                    best_move = Some((eval, player_move.0, player_move.1));
                }
                board.undo_move(player_move, Player::X);
                print!("{} ", eval);
            }
            println!("");
            return best_move;
        },
        Player::O => {
            let mut min_eval = 1000;
            let mut best_move = None;
            for player_move in board.moves() {
                board.apply_move(player_move, Player::O);
                let eval = minimax(&mut HashMap::new(), board, true, depth);
                if eval < min_eval {
                    min_eval = eval;
                    best_move = Some((eval, player_move.0, player_move.1));
                }
                board.undo_move(player_move, Player::O);
            }
            return best_move;
        }
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
        match find_best_move(board, player, _time_limit) {
            Some(best_move) => best_move,
            None => panic!("No best move found!")
        }
        // (0, 0, 0)
    }
}
