// Imports
use crate::solution::{
    engine::iterative_search,
    movement::{
        get_eval,
        get_index
    },
    player::Player,
    tile_map::{
        TILE_MAP_LINE,
        enable_bit_map_o,
        enable_bit_map_w,
        enable_bit_map_x
    }
};

pub struct SolutionAgent {}
impl tic_tac_toe_stencil::agents::Agent for SolutionAgent {
    fn solve(
        board: &mut tic_tac_toe_stencil::board::Board,
        player: tic_tac_toe_stencil::player::Player,
        duration: u64
    ) -> (i32, usize, usize) {
        // Creates tile map
        let cells = board.get_cells();
        let mut tile_map = (0, 0, 0);
        for y in 0..TILE_MAP_LINE {
            for x in 0..TILE_MAP_LINE {
                let index = y * TILE_MAP_LINE + x;
                match cells[y][x] {
                    tic_tac_toe_stencil::board::Cell::X => enable_bit_map_x(&mut tile_map, index),
                    tic_tac_toe_stencil::board::Cell::O => enable_bit_map_o(&mut tile_map, index),
                    tic_tac_toe_stencil::board::Cell::Wall => enable_bit_map_w(&mut tile_map, index),
                    _ => ()
                }
            }
        }

        // Searches movement
        let engine = match player {
            tic_tac_toe_stencil::player::Player::X => Player::X,
            tic_tac_toe_stencil::player::Player::O => Player::O
        };
        let movement = iterative_search(&mut tile_map, &engine, 20, duration as u128 - 25);
        
        // Resolves movement
        let index = get_index(&movement);
        let eval = get_eval(&movement);
        let (x, y) = (index % TILE_MAP_LINE, index / TILE_MAP_LINE);
        (eval as i32, y, x)
    }
}
