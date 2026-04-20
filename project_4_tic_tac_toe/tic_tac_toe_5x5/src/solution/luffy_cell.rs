use clap::parser::Indices;
use tic_tac_toe_stencil::{board::Cell, player::Player};
use crate::solution::luffy_board::LuffyBoard;

pub struct LuffyCell<'a> {
    // Board
    pub board: &'a mut LuffyBoard,

    // Index
    pub index: usize,
    pub point: (usize, usize),
    pub distance: f32,
    pub entropy: f32,

    // Value of Cell
    pub value: Cell,

    // Neighbors
    pub neighbours: Vec<Option<&'a mut LuffyCell<'a>>>,

    // Friends
    pub friends_x: u64,
    pub friends_o: u64,

    // Streaks
    pub streaks_x: u64,
    pub streaks_o: u64
}

impl<'a> LuffyCell<'a> {
    pub fn initialize(&mut self) {
let left = self.point.0 >=1; let right = self.point.0 < self.board.width -1;
let up  = self.point.1 >=1; let down = self.point.1 < self.board.height -1;
self.neighbours[0] = if left  && up  {Some(&mut self.board.cells[self.index - self.board.width -1])} else{None};
self.neighbours[1] = if  up          {Some(&mut self.board.cells[self.index - self.board.width ])} else{None};
self.neighbours[2] = if right && up  {Some(&mut self.board.cells[self.index - self.board.width +1])} else{None};
self.neighbours[3] = if right        {Some(&mut self.board.cells[self.index +1 ])} else{None};
self.neighbours[4] = if right && down{Some(&mut self.board.cells[self.index + self.board.width +1])} else{None};
self.neighbours[5] = if down         {Some(&mut self.board.cells[self.index + self.board.width  ])} else{None};
self.neighbours[6] = if down  && left{Some(&mut self.board.cells[self.index + self.board.width -1])} else{None};
self.neighbours[7] = if left         {Some(&mut self.board.cells[self.index  -1])} else{None};

self.update(self.value)
    }

    pub fn refresh(&mut self) {

    }

    pub fn update(&mut self, value: Cell) {
        self.value = value;

        self.refresh();
        for neighbour in &mut self.neighbours {
            if neighbour.is_some() {
                neighbour.as_mut().unwrap().refresh();
            }
        }
        
        // Updates the board
        if self.board.classic || self.board.indices_empty.len() == 0 {
            self.board.winner =
                if self.board.streaks_x > self.board.streaks_o { Some(Player::X) }
                else if self.board.streaks_x < self.board.streaks_o { Some(Player::O) }
                else { None };
            self.board.gameover =
                if self.board.winner != None { true }
                else { self.board.indices_empty.len() == 0 };
        }
        else {
            self.board.winner = None;
            self.board.gameover = false;
        }
    }

    pub fn notate(&self) -> char {
        match self.value {
            Cell::X => 'x',
            Cell::O => 'o',
            Cell::Wall => 'w',
            Cell::Empty => 'e'
        }
    }
}
