use core::{f32};
use std::{cmp, collections::{HashMap, HashSet}};
use std::time::{Instant};
use tic_tac_toe_stencil::{board::Cell, player::Player};
use crate::solution::luffy_cell::LuffyCell;

pub struct LuffyBoard {
    // Size
    pub height: usize,
    pub width: usize,
    pub size: usize,

    // Center
    pub center: (f32, f32),

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
    pub cells: Vec<LuffyCell>,

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
}

impl LuffyBoard {
    pub fn new(position: Vec<Cell>, height: usize, width: usize, classic: bool) -> Self {
        // Creates board
        let mut board = LuffyBoard {
            // Size
            height: height,
            width: width,
            size: height * width,

            // Center
            center: ((width as f32 - 1.0) / 2.0, (height as f32 - 1.0) / 2.0),

            // Indices
            indices_x: HashSet::new(),
            indices_o: HashSet::new(),
            indices_empty: HashSet::new(),

            // Transpositions
            transpositions: HashMap::new(),

            // Priorities
            priorities: HashMap::new(),

            // Classic
            classic: classic,

            // Cells
            cells: Vec::with_capacity(height * width),

            // Streaks
            streaks_x: 0,
            streaks_o: 0,

            // Distance
            distance_x: 0.0,
            distance_o: 0.0,

            // Entropy
            entropy_x: 0.0,
            entropy_o: 0.0,
            
            // Winner
            winner: None,
            gameover: false
        };

        // Populates board
        for index in 0..position.len() {
            // Fetches value
            let value = &position[index];

            // Inserts cell
            board.cells.push(LuffyCell::new(board.width, board.height, board.center, index, value.clone()));

            // Inserts indices
            match value {
                Cell::X => { board.indices_x.insert(index); },
                Cell::O => { board.indices_o.insert(index); },
                Cell::Empty => { board.indices_empty.insert(index); },
                _ => ()
            }
        }

        // Initializes cells
        for index in 0..(board.size as usize) { board.link_cell_neighbors(index); }
        for index in 0..(board.size as usize) { board.refresh_cell_references(index); }

        // Returns board
        board
    }

    pub fn link_cell_neighbors(&mut self, index: usize) {
        // Fetches size
        let width = self.width as i32;
        let height = self.height as i32;
        
        // Fetches cell
        let cell = &mut self.cells[index];
        let index = cell.index as i32;
        let x = cell.point.0 as i32;
        let y = cell.point.1 as i32;

        // Links neighbors
        let left  = x >= 1; let right = x < width - 1;
        let up    = y >= 1; let down  = y < height - 1;
        cell.neighbours.push(if left  && up   { Some((index - width - 1) as usize) } else{ None });
        cell.neighbours.push(if          up   { Some((index - width) as usize) }     else{ None });
        cell.neighbours.push(if right && up   { Some((index - width + 1) as usize) } else{ None });
        cell.neighbours.push(if right         { Some((index + 1) as usize) }         else{ None });
        cell.neighbours.push(if right && down { Some((index + width + 1) as usize) } else{ None });
        cell.neighbours.push(if          down { Some((index + width) as usize) }     else{ None });
        cell.neighbours.push(if left  && down { Some((index + width - 1) as usize) } else{ None });
        cell.neighbours.push(if left          { Some((index - 1) as usize) }         else{ None });
    }

    pub fn refresh_cell_references(&mut self, index: usize) {
        // Fetches cell data
        let cell = &self.cells[index];
        let cell_streaks_x = cell.streaks_x;
        let cell_streaks_o = cell.streaks_o;
        let cell_neighbors: Vec<Option<Cell>> = cell.neighbours.clone().iter().map(|neighbor| {
            match neighbor {
                Some(index) => Some(self.cells[*index].value.clone()),
                None => None
            }
        }).collect();

        // Resets fields
        self.streaks_x -= cell_streaks_x;
        self.streaks_o -= cell_streaks_o;

        // Refreshes friends
        let cell = &mut self.cells[index];
        cell.friends_x = 0;
        cell.friends_o = 0;
        for direction in 0..8 {
            let neighbour = &cell_neighbors[direction];
            match neighbour {
                Some(value) => match value {
                    Cell::X => { cell.friends_x += 1; },
                    Cell::O => { cell.friends_o += 1; }
                    _ => ()
                },
                None => continue
            }
        }

        // Refreshes streaks
        cell.streaks_x = 0;
        cell.streaks_o = 0;
        match cell.value {
            Cell::X => {
                for direction in 0..4 {
                    // Checks neighbor
                    let neighbour = &cell_neighbors[direction];
                    if neighbour.is_none() { continue; }
                    if neighbour.as_ref().unwrap().clone() != cell.value { continue; }

                    // Checks opposite
                    let opposite = &cell_neighbors[direction + 4];
                    if opposite.is_none() { continue; }
                    if opposite.as_ref().unwrap().clone() != cell.value { continue; }

                    // Increments streaks
                    cell.streaks_x += 1;
                }
            },
            Cell::O => {
                for direction in 0..4 {
                    // Checks neighbor
                    let neighbour = &cell_neighbors[direction];
                    if neighbour.is_none() { continue; }
                    if neighbour.as_ref().unwrap().clone() != cell.value { continue; }

                    // Checks opposite
                    let opposite = &cell_neighbors[direction + 4];
                    if opposite.is_none() { continue; }
                    if opposite.as_ref().unwrap().clone() != cell.value { continue; }

                    // Increments streaks
                    cell.streaks_o += 1;
                }
            },
            _ => ()
        }

        // Updates board streaks
        self.streaks_x += cell.streaks_x;
        self.streaks_o += cell.streaks_o;
    }
    
    pub fn update_cell_value(&mut self, index: usize, value: Cell) {
        // Updates cell value
        self.cells[index].value = value;

        // Refreshes cell
        self.refresh_cell_references(index);

        // Refreshes neighbors
        let neighbors = self.cells[index].neighbours.clone();
        for neighbor in neighbors {
            if let Some(neighbor_index) = neighbor {
                self.refresh_cell_references(neighbor_index);
            }
        }

        // Updates winner
        if self.classic || self.indices_empty.len() == 0 {
            self.winner =
                if self.streaks_x > self.streaks_o { Some(Player::X) }
                else if self.streaks_x < self.streaks_o { Some(Player::O) }
                else { None };
            self.gameover =
                if self.winner != None { true }
                else { self.indices_empty.len() == 0 };
        }
        else {
            self.winner = None;
            self.gameover = false;
        }
    }

    pub fn evaluate_heuristics(&self) -> f32 {
        // Checks gameover
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
        ((self.streaks_x.pow(2) as f32) - (self.streaks_o.pow(2) as f32)) - ((self.entropy_x + self.distance_x / 4.0) - (self.entropy_o + self.distance_o / 4.0))
    }

    pub fn generate_moves(&self, player: Player) -> Vec<usize> {
        // Generates moves
        let priority = self.priorities.get(&self.notate());
        let mut empties = self.indices_empty.clone().into_iter().collect::<Vec<usize>>() as Vec<usize>;
        empties.sort_by(|index_a, index_b| {
            // Bumps priority
            if let Some(index_priority) = priority {
                if index_a == index_priority { return cmp::Ordering::Less; }
                if index_b == index_priority { return cmp::Ordering::Greater; }
            }

            // Sorts cells
            let cell_a = &self.cells[*index_a];
            let cell_b = &self.cells[*index_b];
            match player {
                Player::X => {
                    // Sorts entropy
                    let delta_entropy = cell_a.entropy - cell_b.entropy;
                    if delta_entropy < 0.0 { return cmp::Ordering::Less; }
                    if delta_entropy > 0.0 { return cmp::Ordering::Greater; }
                    
                    // Sorts friends
                    let delta_friends = cell_b.friends_x as i32 - cell_a.friends_x as i32;
                    if delta_friends < 0 { return cmp::Ordering::Less; }
                    if delta_friends > 0 { return cmp::Ordering::Greater; }

                    // Sorts distance
                    let delta_distance = cell_a.distance - cell_b.distance;
                    if delta_distance < 0.0 { return cmp::Ordering::Less; }
                    if delta_distance > 0.0 { return cmp::Ordering::Greater; }

                    // Returns equal
                    return cmp::Ordering::Equal;
                },
                Player::O => {
                    // Sorts entropy
                    let delta_entropy = cell_a.entropy - cell_b.entropy;
                    if delta_entropy < 0.0 { return cmp::Ordering::Less; }
                    if delta_entropy > 0.0 { return cmp::Ordering::Greater; }
                    
                    // Sorts friends
                    let delta_friends = cell_b.friends_o as i32 - cell_a.friends_o as i32;
                    if delta_friends < 0 { return cmp::Ordering::Less; }
                    if delta_friends > 0 { return cmp::Ordering::Greater; }

                    // Sorts distance
                    let delta_distance = cell_a.distance - cell_b.distance;
                    if delta_distance < 0.0 { return cmp::Ordering::Less; }
                    if delta_distance > 0.0 { return cmp::Ordering::Greater; }

                    // Returns equal
                    return cmp::Ordering::Equal;
                }
            }
        });
        return empties;

    }

    pub fn iterative_search(&mut self, player: Player, plies: u64, timeout: u64) -> (usize, f32) {
        // Panics if game over
        if self.gameover {
            panic!("The game is over... There are no legal moves left to search.");
        }

        // Finds optimal move
        let epoch = Instant::now();
        let mut index = self.size; let mut eval = 0.0;
        for depth in 0..plies * 2 {
            // Cancels if timed out
            let canceled= Instant::now().duration_since(epoch).as_secs() >= timeout;
            if canceled { break; };

            // Clears transpositions
            self.transpositions.clear();
            
            // Updates results
            let results = self.recursive_search(player, f32::NEG_INFINITY, f32::INFINITY, epoch, timeout, depth);
            index = results.0; eval = results.1;
        }

        // Returns best
        (index,eval)
    }

    pub fn recursive_search(&mut self, player: Player, mut alpha: f32, mut beta: f32, epoch: Instant, timeout: u64, depth: u64) -> (usize, f32) {
        // Checks transposition
        let notation = self.notate();
        if self.transpositions.contains_key(&notation) {
            return (*self.priorities.get(&notation).unwrap(), *self.transpositions.get(&notation).unwrap());
        }

        // Performs minimax
        match player {
            // Maximizes
            Player::X => {
                // Finds optimal move
                let mut index = self.size as usize;
                let mut eval = f32::NEG_INFINITY;
                for available in self.generate_moves(player) {
                    // Does move
                    let cell_old = &mut self.cells[available];
                    self.indices_x.insert(available);
                    self.indices_empty.remove(&available);
                    self.distance_x += cell_old.distance;
                    self.entropy_x += cell_old.entropy;
                    self.update_cell_value(available, Cell::X);

                    // Calculates heuristics
                    let heuristics =
                        if self.gameover || depth == 0 { self.evaluate_heuristics() }
                        else { self.recursive_search(Player::O, alpha, beta, epoch, timeout, depth - 1).1 };
                    if heuristics > eval || index == self.size { index = available; }
                    eval = eval.max(heuristics);
                    alpha = alpha.max(heuristics);

                    // Undoes move
                    let cell_new = &mut self.cells[available];
                    self.indices_x.remove(&available);
                    self.indices_empty.insert(available);
                    self.distance_x -= cell_new.distance;
                    self.entropy_x -= cell_new.entropy;
                    self.update_cell_value(available, Cell::Empty);

                    // Prunes branch
                    let canceled= Instant::now().duration_since(epoch).as_secs() >= timeout;
                    if canceled { break; };
                    if beta <= alpha { break; }
                }

                // Resolves minimax
                self.transpositions.insert(notation.clone(), eval);
                self.priorities.insert(notation.clone(), index);
                return (index, eval);
            },

            // Minimizes
            Player::O => {
                // Finds optimal move
                let mut index = self.size;
                let mut eval = f32::INFINITY;
                for available in self.generate_moves(player) {
                    // Does move
                    let cell_old = &mut self.cells[available];
                    self.indices_o.insert(available);
                    self.indices_empty.remove(&available);
                    self.distance_o += cell_old.distance;
                    self.entropy_o += cell_old.entropy;
                    self.update_cell_value(available, Cell::O);

                    // Calculates heuristics
                    let heuristics =
                        if self.gameover || depth == 0 { self.evaluate_heuristics() }
                        else { self.recursive_search(Player::X, alpha, beta, epoch, timeout, depth - 1).1 };
                    if heuristics < eval || index == self.size { index = available; }
                    eval = eval.min(heuristics);
                    beta = beta.min(heuristics);

                    // Undoes move
                    let cell_new = &mut self.cells[available];
                    self.indices_o.remove(&available);
                    self.indices_empty.insert(available);
                    self.distance_o -= cell_new.distance;
                    self.entropy_o -= cell_new.entropy;
                    self.update_cell_value(available, Cell::Empty);

                    // Prunes branch
                    let canceled= Instant::now().duration_since(epoch).as_secs() >= timeout;
                    if canceled { break; };
                    if beta <= alpha { break; }
                }

                // Resolves minimax
                self.transpositions.insert(notation.clone(), eval);
                self.priorities.insert(notation.clone(), index);
                return (index, eval);
            }
        }
    }

    pub fn notate(&self) -> String {
        // Creates notation
        let mut notation = String::new();
        for cell in &self.cells {
            notation.push(cell.notate());
        }
        notation
    }
}
