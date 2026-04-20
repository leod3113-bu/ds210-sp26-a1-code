use core::f32;
use std::{collections::{HashMap, HashSet}, time::SystemTime};
use tic_tac_toe_stencil::{board::Cell, player::Player};
use crate::solution::luffy_cell::LuffyCell;

pub struct LuffyBoard {
    // Size
    pub height: usize,
    pub with: usize,
    pub size: usize,

    // Center
    pub center: (usize, usize),

    // Indices lookups
    pub indices_x: HashSet<usize>,
    pub indices_o: HashSet<usize>,
    pub indices_empty: HashSet<usize>,

    // Transpositions (Repeated Board Positions)
    pub transpositions: HashMap<String, f32>,

    // Priorities (Last Best Moves)
    pub priorities: HashMap<String, usize>,
    
    // Oneshot
    pub classic: bool, // One Streak = Win for 3x3
    
    // Cells
    pub cells: Vec<LuffyCell>,

    pub streaks_x: u64,

    pub streaks_o: u64,

    pub distance_x: f32, //to prioritize top left corners optimizes runtime
    
    pub distance_o: f32,
    
    pub entropy_x: f32, 

    pub entropy_o: f32,


    // Game Terminal States
    pub winner: Option<Player>,
    pub gameover: bool
}

impl LuffyBoard {
    pub fn evaluate_heuristics(&self) -> f32 {
        if self.gameover {
            return match self.winner {
                Some(winner) => match winner {
                    Player::X => f32::INFINITY,
                    Player::O => f32::NEG_INFINITY
                },
                None => 0.0
            }     
        }
    
        let mut x_streaks = 0;
        for index in &self.indices_x {
            let cell = &self.cells[*index];
            x_streaks += cell.streaks.0;
        }

        let mut o_streaks = 0;
        for index in &self.indices_o {
            let cell = &self.cells[*index];
            o_streaks += cell.streaks.1;
        }
        return (x_streaks - o_streaks) as f32;
    }

    pub fn generate_moves(&mut self) -> Vec<usize> {
        vec![]
    }

    pub fn iterative_search(&mut self, plies: u64) -> (usize, f32) {
        // Panics if game over
        if self.gameover {
            panic!("The game is over... There are no legal moves left to search.");
        }

        // Finds best move
        self.since = SystemTime::now();
        let mut best = None;
        let mut eval = 0.0;
        for depth in 0..(plies * 2) {
            // Cancels if timed out
            let elapsed = SystemTime::now().duration_since(self.since).unwrap().as_secs();
            if elapsed > self.timeout {
                break;
            }

            // Clears transposition between each search
            self.transpositions.clear();

            // Updates best result
            let best_result = self.find_best_move(f32::INFINITY, -f32::INFINITY, depth);
            best = Some(best_result.0);
            eval = best_result.1;
        }

        // Returns best result
        (best.unwrap(), eval)
    }

    pub fn find_best_move(&mut self, mut alpha: f32, mut beta: f32, depth: u64) -> (usize, f32) {
        (0, 0.0)

    }

    pub fn notate(&self) -> String {
        let mut notation = String::new();
        for cell in &self.cells {
            notation.push(cell.notate());
        }
        notation
    }
}
