use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board};
use tic_tac_toe_stencil::player::Player;
use crate::solution::luffy_board::{LuffyBoard};


// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // Parses cells
        let cells = board.get_cells();
        let height = cells.len();
        let width = cells[0].len();

        // Creates position
        let mut position = Vec::with_capacity(height * width);
        for row in cells {
            for cell in row {
                position.push(cell.clone());
            }
        }

        // Creates luffy board
        let mut luffy_board = LuffyBoard::new(position, height, width, false);

        // Finds best
        let results = luffy_board.iterative_search(player, 20, 1);
        let index = results.0;
        let evaluation = results.1;
        
        // Returns results
        let x = index % width; let y = (index - x) / height;
        (evaluation as i32, y, x)
    }
}





