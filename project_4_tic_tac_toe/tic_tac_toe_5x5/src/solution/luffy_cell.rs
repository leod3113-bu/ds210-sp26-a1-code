use tic_tac_toe_stencil::{board::Cell};

pub struct LuffyCell {
    // Index
    pub index: usize,
    pub point: (usize, usize),
    pub distance: f32,
    pub entropy: f32,

    // Value of Cell
    pub value: Cell,

    // Neighbors
    pub neighbours: Vec<Option<usize>>,

    // Friends
    pub friends_x: u64,
    pub friends_o: u64,

    // Streaks
    pub streaks_x: u64,
    pub streaks_o: u64
}

impl LuffyCell {
    pub fn new(width: usize, height: usize, center: (f32, f32), index: usize, value: Cell) -> Self {
        // Parses coordinates
        let x = (index as i32) % (width as i32);
        let y = ((index as i32) - x) / (height as i32);

        // Creates cell
        let cell = LuffyCell {
            // Index
            index: index,
            point: (x as usize, y as usize),
            distance: ((x.pow(2) + (y as i32).pow(2)) as f32).sqrt(),
            entropy: (((x as f32) - center.0).powf(2.0) + ((y as f32) - center.1).powf(2.0)).sqrt(),

            // Value
            value: value,

            // Neighbors
            neighbours: Vec::with_capacity(8),

            // Friends
            friends_x: 0,
            friends_o: 0,

            // Streaks
            streaks_x: 0,
            streaks_o: 0
        };

        // Returns cell
        cell
    }

    pub fn notate(&self) -> char {
        // Returns notation
        match self.value {
            Cell::X => 'x',
            Cell::O => 'o',
            Cell::Wall => 'w',
            Cell::Empty => 'e'
        }
    }
}
