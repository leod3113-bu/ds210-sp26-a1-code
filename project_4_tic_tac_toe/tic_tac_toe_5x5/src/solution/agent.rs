use std::arch::aarch64::float32x2_t;
use std::collections::{HashMap, HashSet};

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{self, Board, Cell};
use tic_tac_toe_stencil::player::Player;


// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        unimplemented!("Not yet implemented")
    }
}


pub struct OurBoard{
    height: usize,
    with: usize,
    size: usize,
    center: (usize,usize),
    indicesx: HashSet<usize>,
    indiceso: HashSet<usize>,
    availables: HashSet<usize>,
    transpositions : HashMap<String,(usize,i32)>,
    priorities :HashMap<String, usize>,
    
    oneshot: bool, //so we are within the matrtix
    cells: Vec<OurCell>,
    timeout: usize,
    since: usize,

    player: Player,

    straks: (usize,usize),
    progression: f32,
    winner: Option<Player>,
    gameover: bool, 
}

pub struct OurCell {
    board: OurBoard,
    index: usize,
    point: (usize,usize),
    neighbours: (

        Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>,
        Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>,
        Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>,
        Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>, Option<Box<OurCell>>,
    ),
    value: Cell,
    streaks: (usize,usize),
}
