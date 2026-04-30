// Imports
use crate::solution::tile_map::TILE_MAP_SIZE;

// Typedef
pub type Movement = (usize, f32);

// Helpers
pub fn create_movement(index: usize, eval: f32) -> Movement {
    (index, eval)
}

pub fn create_illegal_movement(eval: f32) -> Movement {
    (TILE_MAP_SIZE, eval)
}

pub fn get_index(movement: &Movement) -> usize {
    movement.0
}

pub fn get_eval(movement: &Movement) -> f32 {
    movement.1
}

pub fn set_index(movement: &mut Movement, index: usize) {
    movement.0 = index
}

pub fn set_eval(movement: &mut Movement, eval: f32) {
    movement.1 = eval
}

pub fn is_better(movement: &Movement, eval: f32) -> bool {
    eval > get_eval(movement)
}

pub fn is_worse(movement: &Movement, eval: f32) -> bool {
    eval < get_eval(movement)
}

pub fn is_legal(movement: &Movement) -> bool {
    get_index(movement) < TILE_MAP_SIZE
}
