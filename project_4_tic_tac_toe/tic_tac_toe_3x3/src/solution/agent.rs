use std::cmp;
use std::collections::{HashMap};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;
use crate::layout::Layout3x3;

// Your solution solution.
pub struct SolutionAgent {}

// If the one of the board's transposition is already evaluated, return that score
pub fn get_eval<'a>(evals: &'a mut HashMap<String, i32>, board: &Board) -> Option<&'a i32> {
    // First we check the 4 rotations of the board
    // If we find any, return it
    let mut mut_board = rotate_board(board);
    for _ in 0..4 {
        let hash = &hash_board(&mut_board);
        if evals.contains_key(hash) { return evals.get(hash); } // Rotating 360 degrees and 'snapshotting' and checking if composition already exists
        mut_board = rotate_board(&mut_board);
    }

    // Otherwise, we flip the board by its vertical axis and check the 4 rotations again
    // If we find any, return it
    mut_board= flip_board(&mut_board);
    for _ in 0..4 {
        let hash = &hash_board(&mut_board);
        if evals.contains_key(hash) { return evals.get(hash); } // We can do the exact same steps as before
        mut_board = rotate_board(&mut_board);
    }

    // Otherwise, return none
    None
}

// Converts the board into a string notation
pub fn hash_board(board: &Board) -> String {
    // We need to get all of the cells of the board
    let two_d_map = board.get_cells();

    // And create an empty string that will hold our notation
    let mut hash_brown_fries = String::new();

    // For every single cell
    for vector in two_d_map {
        for cell in vector{
            match cell {
                // If X has played in this cell, we add an "X" character
                Cell::X => hash_brown_fries.push_str("X"),

                // Or if O has played in this cell, we add an "O" character
                Cell::O => hash_brown_fries.push_str("O"),

                // Otherwise, we add an underscore to indicate that this cell is empty
                Cell::Empty =>hash_brown_fries.push_str("_"),

                // We add a catch-all (ie. walls) for cells that we don't expect to find in this 3x3 board
               _=> panic!("Invalid cell!")
            }

        }
    }

    // Then we return the board hash
    hash_brown_fries // and some ketchup and mickidies!!
}

// Rotates board clock-wise
pub fn rotate_board(board: &Board) -> Board {
    // For every cells
    let cells = board.get_cells();

    // First we create an empty board
    let mut rotated = Board::new(Layout3x3 {});
    
    // Then we map the coordinates to the rotated board
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

            // Of course, since Rust hates us, we need to transform Cell enum to Player enum
            match cell_to_player(cells, m) {
                Some(player) => rotated.apply_move(m_prime, player),
                None => ()
            };
        }
    }

    // Finally we return the rotated board
    rotated
}

// Flips board horizontally
pub fn flip_board(board: &Board) -> Board {
    // For every cells
    let cells = board.get_cells();
    
    // First we create an empty board
    let mut flipped = Board::new(Layout3x3 {});
    
    // Then we mirror the cells by the board's vertical axis
    for y in 0..cells.len() {
        for x in 0..cells.len() {
            let m = (y, x);
            let m_prime = (y, 2 - x);

            // Cell enum -> Player enum
            match cell_to_player(cells, m) {
                Some(player) => flipped.apply_move(m_prime, player),
                None => ()
            }
        }
    }

    // Returns the flipped board
    flipped
}

// Turns Cell enum to Player enum at cell location
pub fn cell_to_player(cells: &Vec<Vec<Cell>>, m: (usize, usize)) -> Option<Player> {
    // Just a regular match case
    match cells[m.0][m.1] {
        Cell::X => Some(Player::X),
        Cell::O => Some(Player::O),
        _ => None
    }
}

// Maximizing function for X
pub fn minimax(evals: &mut HashMap<String, i32>, board: &mut Board, maximizing: bool, depth: u64) -> i32 {
    // If the board has already been evaluated, skip the entire process since we know the answer already
    if let Some(eval) = get_eval(evals, board) {
        return *eval;
    }

    // Otherwise, we hash the board
    // (hashing = creating a string notation / identifier so we can find this exact layout later)
    let hash = hash_board(board);

    // If the game is done, either by a tie or a win
    if board.game_over() {
        // Calculate the score using the given function provided to us
        // score > 0 = X won, score < 0 = O won, score == 0 = it's a tie
        let score = board.score();

        // If it is a tie, return 0
        if score == 0 {
            evals.insert(hash, 0);
            return 0;
        }

        // If it is a win, return positive for X, negative for O
        let eval = if score > 0 { 10 } else { -10 };
        evals.insert(hash, eval);
        return eval;
    }

    // Does nothing but a sanity check - Leo Deng
    // Don't worry too much about this for the code review
    if depth == 0 {
        return 0;
    }

    // 'maximizing' is true if we are evaluating X's situation
    if maximizing {
        // Assume that this move is bad
        // (we have to, otherwise we are always assuming a move is good, so the bad ones would never be filtered out)
        let mut max_eval = -1000;

        // For every valid move (that's what board.moves() gives)
        for player_move in board.moves() {
            // We apply the move so we can see what happens
            board.apply_move(player_move, Player::X);

            // From this position, we calculate how well this move does for X
            let eval = minimax(evals, board, false, depth - 1);

            // We want X to win, so we want the score to be as high as possible
            // cmp::max just gives us the bigger number of the two
            // If our original move is better, keep the original move (ie. max_eval)
            // Otherwise if this new move is better than the original, use the new one (ie. eval)
            max_eval = cmp::max(max_eval, eval);

            // Obviously we want to undo the move after we are done analyzing
            // Otherwise we would be making illegal moves by playing twice
            board.undo_move(player_move, Player::X);
        }

        // After every move has been evaluated, we have a rough estimation of how well this move is
        // So we return this eval score as our result
        evals.insert(hash, max_eval);
        return max_eval;
    }

    // 'maximizing' is false if we are evaluating O's situation
    // Remember, we want X to win, so we want to minimize O's winning chance

    // The "wanting X to win" is a completely arbitrary decision btw.
    // The way this algorithm works is that we are always assuming we want exactly one person to win
    // Usually, that person is the first player that moves in the game (think always the main character)
    // Not necessarily the player that the agent is playing as
    // Because if we want to optimize from the other side, we can just negate the entire function afterwards
    // To reverse the effect, essentially making it minimize the chances for X and maximizing the chances for O
    // I hope this doesn't confuse you for the code review - Leo Deng
    else {
        // Assume this move is very very good for X (see reasons above)
        let mut min_eval = 1000;

        // For every available move
        for player_move in board.moves() {
            // We apply the move to see what happens
            board.apply_move(player_move, Player::O);

            // We want the find the worse move that O can make
            // (again, see reasons above if you are confused about why we want O to lose)
            let eval = minimax(evals, board, true, depth - 1);
            
            // If the old move was worse (ie. min_eval), keep the original
            // Otherwise, if the new move for O is worse (ie. eval), use the new one
            min_eval = cmp::min(min_eval, eval);

            // Undo the move so we don't make illegal moves by playing twice
            board.undo_move(player_move, Player::O);
        }

        // Now we have an estimate of the lowest score for O
        // So we are going to return it as our result
        evals.insert(hash, min_eval);
        return min_eval;
    }
}

// Finds the best move from a given position
pub fn find_best_move(board: &mut Board, player: Player, depth: u64) -> Option<(i32, usize, usize)> {
    // Now this is the part where we want to maximize for a specific player
    match player {
        // If we are playing as X
        Player::X => {
            // Then we want to find the best move for X in this position
            // This follows the same logic as minimax

            // Assume this move is bad
            let mut max_eval = -1000;
            let mut best_move = None;

            // For every player move available
            for player_move in board.moves() {
                // We try out this move
                board.apply_move(player_move, Player::X);

                // And see how well this move performs
                let eval = minimax(&mut HashMap::new(), board, false, depth);

                // If the score is better
                if eval > max_eval {
                    // We update our 'best_move' variable
                    max_eval = eval;
                    best_move = Some((eval, player_move.0, player_move.1));
                }

                // And undo it so we don't make illegal moves since we are only analyzing for this step
                board.undo_move(player_move, Player::X);
            }

            // Of course, then we return our move as our calculated result
            return best_move;
        },

        // Otherwise, if we are playing as X
        Player::O => {
            // Then we want to minimize the winning chance for X
            // Assume this move is very very good for X
            let mut min_eval = 1000;
            let mut best_move = None;

            // Then our best move is the one that will give the least advantage for X
            // So for every available move that O can make at this time
            for player_move in board.moves() {
                // We apply this move as an experiment
                board.apply_move(player_move, Player::O);

                // Calculate the score for this move
                let eval = minimax(&mut HashMap::new(), board, true, depth);
                
                // If it gives worse results for X, that's good for O
                if eval < min_eval {
                    // So we update our 'best_move' accordingly
                    min_eval = eval;
                    best_move = Some((eval, player_move.0, player_move.1));
                }

                // Undo so we don't accidently create illegal moves
                board.undo_move(player_move, Player::O);
            }

            // And return that best move
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

        // We find the best move for player at this position
        match find_best_move(board, player, _time_limit) {
            // If we found one, we return that best move as our final answer
            Some(best_move) => best_move,

            // Otherwise... something weird happened, since it means we have no more illegal moves
            // Yet... the game is still running? What????
            // It shouldn't happen in theory, but Rust won't be happy about our code if we don't include this None branch
            // So yeah ¯\_(ツ)_/¯
            None => panic!("No best move found!")
        }
    }
}
