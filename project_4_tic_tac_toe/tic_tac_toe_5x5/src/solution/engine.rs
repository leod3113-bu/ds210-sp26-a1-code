// Imports
use std::{collections::HashMap};
use crate::solution::{
    heat_map::{
        compute_heat_map,
        get_affinity,
        get_dominance,
        get_potential
    },
    movement::{
        Movement,
        create_illegal_movement,
        create_movement,
        get_eval,
        get_index,
        is_better,
        is_legal,
        is_worse,
        set_eval,
        set_index
    },
    player::Player,
    set_piece::get_shape_worth,
    set_pieces::SET_PIECES,
    tile_map::{
        TileMap,
        check_bit_map_e,
        count_set_piece,
        disable_bit_map_o,
        disable_bit_map_x, enable_bit_map_o,
        enable_bit_map_x,
        is_tile_map_full
    },
    timeout::{
        Timeout,
        begin_timeout,
        has_timed_out
    }
};

// Engine
pub fn enumerate_availables(tile_map: &TileMap, player: &Player, priority: Option<&usize>) -> Vec<usize> {
    // Computes availables
    let mut heat_maps = [(0, 0, 0); 25];
    let mut availables = Vec::with_capacity(25);
    for index in 0..25 {
        if !check_bit_map_e(tile_map, index) { continue; }
        heat_maps[index] = compute_heat_map(tile_map, player, index);
        availables.push(index);
    }

    // Sorts availables
    availables.sort_by(|index_a, index_b| {
        // Forces priority
        if let Some(index_priority) = priority {
            if *index_a == *index_priority { return std::cmp::Ordering::Less; }
            if *index_b == *index_priority { return std::cmp::Ordering::Greater; }
        }

        // Compares heat maps
        let heat_map_a = &heat_maps[*index_a];
        let heat_map_b = &heat_maps[*index_b];
        get_potential(heat_map_b).cmp(&get_potential(heat_map_a))
            .then_with(|| get_affinity(heat_map_b).cmp(&get_affinity(heat_map_a)))
            .then_with(|| get_dominance(heat_map_b).cmp(&get_dominance(heat_map_a)))
            .then_with(|| index_a.cmp(&index_b))
    });

    // Resolves availables
    availables
}

pub fn evaluate_position(tile_map: &TileMap) -> f32 {
    // Evaluates final
    if is_tile_map_full(tile_map) {
        // Computes score
        let mut score = 0;
        for index in 0..4 {
            let set_piece = SET_PIECES[index];
            score += count_set_piece(tile_map, &Player::X, &set_piece) as i8;
            score -= count_set_piece(tile_map, &Player::O, &set_piece) as i8;
        }

        // Resolves eval
        if score > 0 { f32::INFINITY } else if score < 0 { f32::NEG_INFINITY } else { 0.0 }
    }

    // Evaluates partial
    else {
        // Computes eval
        let mut eval = 0.0;
        for set_piece in SET_PIECES {
            eval += (count_set_piece(tile_map, &Player::X, &set_piece) as f32) * get_shape_worth(&set_piece);
            eval -= (count_set_piece(tile_map, &Player::O, &set_piece) as f32) * get_shape_worth(&set_piece);
        }
    
        // Resolves eval
        eval
    }
}

pub fn iterative_search(tile_map: &mut TileMap, player: &Player, plys: u8, duration: u128) -> Movement {
    // Checks game over
    if is_tile_map_full(tile_map) { panic!("No more legal moves to search."); }

    // Initializes timeout
    let timeout = begin_timeout(duration);

    // Initializes caches
    let mut transposes = HashMap::new();
    let mut priorities = HashMap::new();

    // Computes ply offset
    let offset = match player { Player::X => 1, Player::O => 0 };

    // Searches movement
    let mut decision = create_illegal_movement(0.0);
    for ply in 0..plys {
        // Clears transposes
        transposes.clear();

        // Initiates search
        let movement = recursive_search(
            // Tile map
            tile_map,

            // Player
            player,

            // Caches
            &mut transposes,
            &mut priorities,

            // Pruner
            f32::NEG_INFINITY,
            f32::INFINITY,

            // Depth
            ply * 2 + offset,

            // Timeout
            &timeout
        );

        // Checks timeout
        if has_timed_out(&timeout) { break; }

        // Checks illegal movmeent
        if !is_legal(&movement) { break; }

        // Updates decision
        decision = movement;
    }

    // Resolves decision
    if !is_legal(&decision) { panic!("Decision movement is illegal."); }
    decision
}

pub fn recursive_search(
    // Tile map
    tile_map: &mut TileMap,

    // Player
    player: &Player,

    // Caches
    transposes: &mut HashMap<(u32, u32, u32), f32>,
    priorities: &mut HashMap<(u32, u32, u32), usize>,

    // Pruner
    mut alpha: f32,
    mut beta: f32,

    // Depth
    depth: u8,

    // Timeout
    timeout: &Timeout
) -> Movement {
    // Checks transposition
    if transposes.contains_key(tile_map) {
        let index = *priorities.get(tile_map).unwrap();
        let evaluation = *transposes.get(tile_map).unwrap();
        return create_movement(index, evaluation);
    }

    // Executes minimax
    match player {
        Player::X => {
            // Searches movement
            let availables = enumerate_availables(tile_map, player, priorities.get(tile_map));
            let remaining = availables.len();
            let mut movement = create_illegal_movement(f32::NEG_INFINITY);
            for index in availables {
                // Checks timeout
                if has_timed_out(timeout) { break; }

                // Does move
                enable_bit_map_x(tile_map, index);

                // Evaluates movement
                let eval = if is_tile_map_full(tile_map) || depth == 0 { evaluate_position(tile_map) } else { recursive_search(
                    // Tile map
                    tile_map,

                    // Player
                    &Player::O,
                    
                    // Caches
                    transposes,
                    priorities,
                    
                    // Pruner
                    alpha,
                    beta,
                    
                    // Depth
                    depth - 1,
                    
                    // Timeout
                    timeout
                ).1 };

                // Updates index
                if is_better(&movement, eval) || !is_legal(&movement) {
                    set_index(&mut movement, index);
                    set_eval(&mut movement, eval);
                }
                alpha = alpha.max(eval);

                // Undoes move
                disable_bit_map_x(tile_map, index);

                // Prunes branch
                if beta <= alpha && remaining > 11 { break; }
            }

            // Updates caches
            transposes.insert(*tile_map, get_eval(&movement));
            priorities.insert(*tile_map, get_index(&movement));

            // Resolves movement
            movement
        },
        Player::O => {
            // Searches movement
            let availables = enumerate_availables(tile_map, player, priorities.get(tile_map));
            let remaining = availables.len();
            let mut movement = create_illegal_movement(f32::INFINITY);
            for index in availables {
                // Checks timeout
                if has_timed_out(timeout) { break; }

                // Does move
                enable_bit_map_o(tile_map, index);

                // Evaluates movement
                let eval = if is_tile_map_full(tile_map) || depth == 0 { evaluate_position(tile_map) } else { recursive_search(
                    // Tile map
                    tile_map,

                    // Player
                    &Player::X,
                    
                    // Caches
                    transposes,
                    priorities,
                    
                    // Pruner
                    alpha,
                    beta,
                    
                    // Depth
                    depth - 1,
                    
                    // Timeout
                    timeout
                ).1 };

                // Updates index
                if is_worse(&movement, eval) || !is_legal(&movement) {
                    set_index(&mut movement, index);
                    set_eval(&mut movement, eval);
                }
                beta = beta.min(eval);

                // Undoes move
                disable_bit_map_o(tile_map, index);

                // Prunes branch
                if beta <= alpha && remaining > 11 { break; }
            }

            // Updates caches
            transposes.insert(*tile_map, get_eval(&movement));
            priorities.insert(*tile_map, get_index(&movement));

            // Resolves movement
            movement
        }
    }
}
