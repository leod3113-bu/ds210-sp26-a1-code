// Imports
use crate::solution::{
    player::Player,
    set_piece::{
        SetPiece,
        get_shape_height,
        get_shape_map_b,
        get_shape_map_e,
        get_shape_map_f,
        get_shape_width
    },
    tile::Tile
};

// Constants
pub const TILE_MAP_LINE: usize = 5;
pub const TILE_MAP_MASK: u32 = 0b0000000_11111_11111_11111_11111_11111;
pub const TILE_MAP_SIZE: usize = 25;

// Typedef
pub type TileMap = (u32, u32, u32);

// Methods
pub fn create_from_board(board: &[Tile; 25]) -> TileMap {
    // Creates tile map
    let mut tile_map: TileMap = (0, 0, 0);

    // Computes tile map
    for index in 0..25 {
        match board[index] {
            Tile::X => enable_bit_map_x(&mut tile_map, index),
            Tile::O => enable_bit_map_o(&mut tile_map, index),
            Tile::W => enable_bit_map_w(&mut tile_map, index),
            _ => ()
        }
    }

    // Resolves tile map
    tile_map
}

pub fn count_set_piece(tile_map: &TileMap, player: &Player, set_piece: &SetPiece) -> u8 {
    // Parses bit maps
    let bit_map_f = get_bit_map_f(tile_map, player);
    let bit_map_b = get_bit_map_b(tile_map, player);
    let bit_map_e = get_bit_map_e(tile_map);

    // Parses shape maps
    let shape_map_f = get_shape_map_f(set_piece);
    let shape_map_b = get_shape_map_b(set_piece);
    let shape_map_e = get_shape_map_e(set_piece);

    // Counts set piece
    let mut count = 0;
    for y in 0..(TILE_MAP_LINE - get_shape_height(set_piece) + 1) {
        for x in 0..(TILE_MAP_LINE - get_shape_width(set_piece) + 1) {
            let offset = y * TILE_MAP_LINE + x;
            if bit_map_f >> offset & shape_map_f != shape_map_f { continue; }
            if bit_map_b >> offset & shape_map_b != shape_map_b { continue; }
            if bit_map_e >> offset & shape_map_e != shape_map_e { continue; }
            count += 1;
        }
    }
    
    // Resolves count
    count
}

// Helpers
pub fn get_bit_map_x(tile_map: &TileMap) -> u32 {
    tile_map.0
}

pub fn get_bit_map_o(tile_map: &TileMap) -> u32 {
    tile_map.1
}

pub fn get_bit_map_w(tile_map: &TileMap) -> u32 {
    tile_map.2
}

pub fn get_bit_map_f(tile_map: &TileMap, player: &Player) -> u32 {
    match player { Player::X => tile_map.0, Player::O => tile_map.1 }
}

pub fn get_bit_map_b(tile_map: &TileMap, player: &Player) -> u32 {
    tile_map.2 | match player { Player::X => tile_map.1, Player::O => tile_map.0 }
}

pub fn get_bit_map_e(tile_map: &TileMap) -> u32 {
    !(tile_map.0 | tile_map.1 | tile_map.2) & TILE_MAP_MASK
}

pub fn check_bit_map_x(tile_map: &TileMap, index: usize) -> bool {
    get_bit_map_x(tile_map) & 1 << index > 0
}

pub fn check_bit_map_o(tile_map: &TileMap, index: usize) -> bool {
    get_bit_map_o(tile_map) & 1 << index > 0
}

pub fn check_bit_map_w(tile_map: &TileMap, index: usize) -> bool {
    get_bit_map_w(tile_map) & 1 << index > 0
}

pub fn check_bit_map_f(tile_map: &TileMap, player: &Player, index: usize) -> bool {
    get_bit_map_f(tile_map, player) & 1 << index > 0
}

pub fn check_bit_map_b(tile_map: &TileMap, player: &Player, index: usize) -> bool {
    get_bit_map_b(tile_map, player) & 1 << index > 0
}

pub fn check_bit_map_e(tile_map: &TileMap, index: usize) -> bool {
    get_bit_map_e(tile_map) & 1 << index > 0
}

pub fn enable_bit_map_x(tile_map: &mut TileMap, index: usize) {
    tile_map.0 |= 1 << index
}

pub fn disable_bit_map_x(tile_map: &mut TileMap, index: usize) {
    tile_map.0 &= !(1 << index)
}

pub fn enable_bit_map_o(tile_map: &mut TileMap, index: usize) {
    tile_map.1 |= 1 << index
}

pub fn disable_bit_map_o(tile_map: &mut TileMap, index: usize) {
    tile_map.1 &= !(1 << index)
}

pub fn enable_bit_map_w(tile_map: &mut TileMap, index: usize) {
    tile_map.2 |= 1 << index
}

pub fn disable_bit_map_w(tile_map: &mut TileMap, index: usize) {
    tile_map.2 &= !(1 << index)
}

pub fn is_tile_map_full(tile_map: &TileMap) -> bool {
    get_bit_map_e(tile_map) == 0
}
