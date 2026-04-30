// Imports
use crate::solution::{
    player::Player,
    tile_map::{
        TILE_MAP_LINE,
        TileMap,
        check_bit_map_b,
        check_bit_map_f
    }
};

// Constants
pub const DIRECTIONS: [(i8, i8); 8] = [
    ( 0,  1), ( 0, -1),
    (-1,  0), ( 1,  0),
    (-1, -1), ( 1,  1),
    ( 1, -1), (-1,  1)
];

// Typedef
pub type HeatMap = (u8, u8, u8);

// Methods
pub fn compute_heat_map(tile_map: &TileMap, player: &Player, index: usize) -> HeatMap {
    // Creates heat map
    let mut heat_map = (0, 0, 0);

    // Creates polarity
    let mut polarity = [0; 8];

    // Computes heat map
    let (x, y) = (index % TILE_MAP_LINE, index / TILE_MAP_LINE);
    for direction in 0..8 {
        // Initializes temporary
        let mut chain = 1;
        let mut affinity = 0;
        let mut dominance = 0;
        
        // Computes coordinates
        let (dx, dy) = DIRECTIONS[direction];
        let (mut fx, mut fy) = (x, y);

        // Marches neighbors
        while fx > 0 && fx < TILE_MAP_LINE - 1 && fy > 0 && fy < TILE_MAP_LINE - 1 {
            (fx, fy) = ((fx as i8 + dx) as usize, (fy as i8 + dy) as usize);
            if check_bit_map_b(tile_map, player, fy * TILE_MAP_LINE + fx) { break; }
            if !check_bit_map_f(tile_map, player, index) { chain = 0; }
            affinity += 1 * chain;
            dominance += 1;
        }

        // Aggregates temporary
        if affinity >= 1 { polarity[direction] = 1; }
        if affinity >= 2 { heat_map.0 += 1; }
        heat_map.1 += affinity;
        heat_map.2 += dominance;
    }

    // Computes polar potentials
    for diection in 0..4 {
        if polarity[diection * 2] == 1 && polarity[diection * 2 + 1] == 1 { heat_map.0 += 1; }
    }

    // Resolves heat map
    heat_map
}

// Helpers
pub fn get_potential(heat_map: &HeatMap) -> u8 {
    heat_map.0
}

pub fn get_affinity(heat_map: &HeatMap) -> u8 {
    heat_map.1
}

pub fn get_dominance(heat_map: &HeatMap) -> u8 {
    heat_map.2
}
