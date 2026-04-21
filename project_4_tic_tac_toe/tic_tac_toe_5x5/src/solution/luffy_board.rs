use core::{f32, time};
use std::{collections::{HashMap, HashSet}, time::SystemTime};
use tic_tac_toe_stencil::{board::Cell, player::Player};
use crate::solution::luffy_cell::LuffyCell;
use std::time::{Instant, Duration};

pub struct LuffyBoard <'a> {
    // Size
    pub height: usize,
    pub width: usize,
    pub size: usize,

    // Center
    pub center: (usize, usize),

    // Indices lookups
    pub indices_x: HashSet<usize>,
    pub indices_o: HashSet<usize>,
    pub indices_empty: HashSet<usize>,

    // Transpositions (Position Evaluations)
    pub transpositions: HashMap<String, f32>,

    // Priorities (Position Best Moves)
    pub priorities: HashMap<String, usize>,
    
    // Classic Mode (Stop game at 1 three-in-a-row)
    pub classic: bool,
    
    // Cells
    pub cells: Vec<LuffyCell<'a>>,

    // Streaks (Number of three-in-a-rows)
    pub streaks_x: u64,
    pub streaks_o: u64,

    // Distance (Distance from Top Left)
    pub distance_x: f32,
    pub distance_o: f32,
    
    // Entropy (Distance from Center)
    pub entropy_x: f32, 
    pub entropy_o: f32,


    // Game Terminal States
    pub winner: Option<Player>,
    pub gameover: bool,

    //Time managment
    pub since: Instant,
}

impl LuffyBoard<'a> {
    
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
    
        // Calculating the streak importance
        (self.streaks_x.pow(2) - self.streaks_o.pow(2)) as f32 - ((self.entropy_x + self.distance_x / 4.0) - (self.entropy_o + self.distance_o / 4.0))
    }

    pub fn generate_moves(&mut self, player: Player) -> Vec<usize> {
        vec![]
    }

    pub fn iterative_search(&mut self, player: Player, plies: u64, timeout: u64) -> (usize, f32) {
        // Panics if game over
        if self.gameover {
            panic!("The game is over... There are no legal moves left to search.");
        }
        let epoch = Instant::now();
        let mut index = self.size; 
        let mut eval = 0.0;
        for depth in 0..plies * 2{
            let cancel= Instant::now().duration_since(epoch).as_secs() >= timeout;
            if cancel {break;};
            self.transpositions.clear();
            let result = self.recursive_search(player, f32::NEG_INFINITY, f32::INFINITY, epoch, timeout, depth);
            index = result.0;
            eval = result.1;
        }
        (index,eval)
        
    }

    pub fn recursive_search(&mut self, player: Player, mut alpha: f32, mut beta: f32, epoch: Instant, timeout: u64, depth: u64) -> (usize, f32) {
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
