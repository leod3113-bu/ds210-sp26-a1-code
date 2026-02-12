use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let mut a = min;
        let mut b = max;
        let mut m = (a + b) / 2;
        while b - a > 1 {
            match player.ask_to_compare(m) {
                -1 => { b = m; }
                1  => { a = m + 1; }
                _  => { break; }
            }
            m = (a + b) / 2;
        }
        return m;
    }
}
