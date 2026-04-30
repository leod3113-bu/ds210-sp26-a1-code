// Imports
use crate::solution::tile_map::TILE_MAP_SIZE;

// Typedef
pub type SetPiece = (u32, u32, u32, usize, usize, f32);

// Methods
pub const fn define_set_piece(shape: [char; 25], width: usize, height: usize, worth: f32) -> SetPiece {
    // Creates set piece
    let mut set_piece = (0, 0, 0, width, height, worth);
    
    // Computes set piece
    let mut index = 0;
    while index < TILE_MAP_SIZE {
        match shape[index] {
            'f' => set_shape_map_f(&mut set_piece, index),
            'b' => set_shape_map_b(&mut set_piece, index),
            'e' => set_shape_map_e(&mut set_piece, index),
            ' ' => (),
            _ => panic!("Unable to parse set piece!")
        }
        index += 1;
    }

    // Resolves set piece
    set_piece
}

// Helpers
pub const fn set_shape_map_f(set_piece: &mut SetPiece, index: usize) {
    set_piece.0 |= 1 << index
}

pub const fn set_shape_map_b(set_piece: &mut SetPiece, index: usize) {
    set_piece.1 |= 1 << index
}

pub const fn set_shape_map_e(set_piece: &mut SetPiece, index: usize) {
    set_piece.2 |= 1 << index
}

pub fn get_shape_map_f(set_piece: &SetPiece) -> u32 {
    set_piece.0
}

pub fn get_shape_map_b(set_piece: &SetPiece) -> u32 {
    set_piece.1
}

pub fn get_shape_map_e(set_piece: &SetPiece) -> u32 {
    set_piece.2
}

pub fn get_shape_width(set_piece: &SetPiece) -> usize {
    set_piece.3
}

pub fn get_shape_height(set_piece: &SetPiece) -> usize {
    set_piece.4
}

pub fn get_shape_worth(set_piece: &SetPiece) -> f32 {
    set_piece.5
}
