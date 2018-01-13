#![feature(inclusive_range_syntax)]

extern crate nalgebra as na;
extern crate num_traits;

use std::env::args;

mod state;
mod markov;

fn main() {
    let num_players: u8 = args().nth(1).unwrap().parse().unwrap();
    let mut players = Vec::with_capacity(num_players as usize);
    for i in 0..num_players {
        players.push(i);
    }


    //let chain = markov::new_markov(num_players);
    //println!("Markov chain: {:?}", chain);

    //let num_chips = num_players * 3;
    //let permuts = permute(&players, num_chips);
    let states = state::create_states(&players);
    println!("Num states: {:?}", states.len());
}
