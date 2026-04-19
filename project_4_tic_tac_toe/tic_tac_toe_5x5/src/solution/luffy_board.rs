use std::collections::{HashMap, HashSet};
use tic_tac_toe_stencil::player::Player;
use crate::solution::luffy_cell::LuffyCell;

pub struct LuffyBoard {
    // Size
    height: usize,
    with: usize,
    size: usize,

    // Center
    center: (usize, usize),

    // Indices lookups
    indices_x: HashSet<usize>,
    indices_o: HashSet<usize>,
    availables: HashSet<usize>,

    // Transpositions (Repeated Board Positions)
    transpositions: HashMap<String, (usize, f32)>,

    // Priorities (Last Best Moves)
    priorities: HashMap<String, usize>,
    
    // Oneshot
    oneshot: bool, // One Streak = Win for 3x3
    
    // Cells
    cells: Vec<LuffyCell>,

    // Timeout for Cutoffs
    timeout: u64,
    since: u64,

    // Current Player
    player: Player,
    
    // Player Scores
    streaks: (u64, u64),

    // Game Terminal States
    winner: Option<Player>,
    gameover: bool
}

impl LuffyBoard {
    fn evaluate_heuristics(&self) -> f32 {
        0.0
    }

    fn generate_moves(&mut self) -> Vec<usize> {
        vec![]
    }

    fn iterative_search(&mut self, plies: u64) -> (usize, f32) {
        (0, 0.0)
    }

    fn find_best_move(mut alpha: f32, mut beta: f32, depth: u64) -> (usize, f32) {
        (0, 0.0)
    }

    fn notate() -> String {
        String::new()
    }
}
