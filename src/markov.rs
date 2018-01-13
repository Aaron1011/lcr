use na::{Dynamic, MatrixArray, MatrixVec, MatrixN};
use na::core::Matrix;
use num_traits::Zero;

use super::state::*;

pub type Markov = Matrix<f32, Dynamic, Dynamic, MatrixVec<f32, Dynamic, Dynamic>>;

// We can move to a state where at most 3 fewer chips
pub const MAX_CHIP_DELTA: u8 = 3;
// At most 3
pub const MAX_PLAYER_DELTA: u8 = 3;

pub const DOT_PROB: f32 = 3.0/6.0;
pub const LEFT_PROB: f32 = 1.0/6.0;
pub const RIGHT_PROB: f32 = 1.0/6.0;
pub const CENTER_PROB: f32 = 1.0/6.0;

fn new_markov(num_players: u8) -> Markov {
    let mut players = Vec::with_capacity(num_players as usize);
    for i in 0..num_players {
        players.push(i);
    }

    let states = create_states(&players);

    let mut markov: MatrixN<f32, Dynamic> = MatrixN::zeros_generic(Dynamic::new(states.len()), Dynamic::new(states.len()));

    for (first_index, first_state) in states.iter().enumerate() {
        for (second_index, second_state) in states.iter().enumerate() {
            markov[(first_index, second_index)] = transition_prob(first_state, second_state, num_players)
        }
    }
    markov
}

#[derive(Debug, Clone)]
struct Change {
    player: u8,
    chips: i16
}

// Tracking legal states transitions in a hashmap would be much faster,
// but would require more than 20 times more memory (each state can transition to 20
// other states)
//
// The following conditions must be met fora state transition to be legal:
// * Turn moves to the next player (wrapping around to the first player at the end)
// * Changes occur to three consecutive players ( again, considering wrapping )
// * Any individual player gains or loses no more than 3 chips
// * No more than three chips move in any way (including to the center)
// * At most three changes occur
//
pub fn transition_prob(state1: &State, state2: &State, players: u8) -> f32 {
    if (state1.turn + 1) % players != state2.turn {
        println!("Bad turns");
        return 0.0
    }

    let turn = state1.turn;

    if state1.assigns.len() != state2.assigns.len() {
        panic!("Mismatch: {:?} {:?}", state1, state2);
    }

    let mut changes: Vec<Change> = Vec::with_capacity(3);

    for (i, (before, after)) in state1.assigns.iter().zip(state2.assigns.iter()).enumerate() {
        if before.chips != after.chips {
            if changes.len() == MAX_PLAYER_DELTA as usize {
                println!("Too many changes: {:?}", changes);
                return 0.0
            }
            if ((before.chips as i16) - (after.chips as i16)).abs() > MAX_CHIP_DELTA as i16 {
                println!("Change too big: {:?} {:?}", before, after);
                return 0.0
            }

            let left_player = if i == 0 { players - 1 } else { (i - 1) as u8 } as u8;
           
            // If there's a change before us, and it wasn't to the player to our left,
            // it's impossible to transition to this state
            /*if !changes.last().map_or(true, |v| v.player == left_player) {

                return 0.0
            }*/

            changes.push(Change { chips: ((after.chips as i16) - (before.chips as i16)), player: i as u8 });
        }
    }

    let net_movement: i16 = changes.iter().map(|c| c.chips).sum();
    if net_movement > MAX_CHIP_DELTA as i16 {
        println!("Bad total: {:?}", net_movement);
        return 0.0
    }

    println!("Initial Changes: {:?}", changes);

    if changes.len() == 0 {
        // No changes means rolling three dots
        return DOT_PROB.powi(3 as i32);
    } else if changes.len() == 1 {
        // This value is always negative
        let num_centers = -changes.last().unwrap().chips;
        if num_centers > 3 || num_centers < 0 {
            return 0.0;
        }

        let num_dots = 3 - num_centers;

        println!("Single change dots: {} centers: {}", num_dots, num_centers);
        
        return CENTER_PROB.powi(num_centers as i32) * DOT_PROB.powi(num_dots as i32)
    } else {

        let mut left_change = None;
        let mut right_change = None;
        let mut player_change = None;

        for c in changes {
            if c.player == turn {
                player_change = Some(c);
            } else if c.player == (turn + 1) % players {
                right_change = Some(c);
            } else if (c.player + 1) % players == turn {
                left_change = Some(c);
            }
        }

        if player_change.is_none() {
            return 0.0;
        }

        //let (change_index, player_change) = changes.iter().enumerate().find(|&(i, c)| c.player == turn).unwrap();
        let (player_number, player_chips) = {
            let player = player_change.clone().unwrap();
            (player.player, player.chips)
        };

        //let left_player  = (if change_index == 0 { changes.get(changes.len() - 1) } else { changes.get((change_index - 1) as usize) } );
        //let right_player = changes.get(((change_index + 1) % changes.len()) as usize);

        /*if !(left_player.map_or(true, |p| ((p.player + 1) % players) == player_number) && right_player.map_or(true, |p| p.player == ((player_number + 1) % players))) {
            println!("Bad player numbers: {:?} {:?}", left_player, right_player);
            return 0.0
        }*/

        let num_left = left_change.map_or(0, |p| p.chips);
        let num_right = right_change.map_or(0, |p| p.chips);

        println!("Final Changes: {:?} {:?} {:?}", num_left, player_change, num_right);

        // Player can never gain chips on their turn, and other players
        // can never gain chips
        if player_chips > 0 || num_left < 0 || num_right < 0 {
            println!("Bad range");
            return 0.0;
        }

        let num_center = -player_chips - (num_left + num_right);
        let num_dots = 3 - (num_left + num_center + num_right);

        println!("Center: {} Dots: {}", num_center, num_dots);

        return LEFT_PROB.powi(num_left as i32) * DOT_PROB.powi(num_dots as i32) * RIGHT_PROB.powi(num_right as i32) * CENTER_PROB.powi(num_center as i32)
    }

    /*right_player = (i + 1) % players;
    if after.turn != i && after.turn != left_player && after.turn != right_player {
        return false
    }*/

    
}

#[cfg(test)]
mod test {

    use markov::*;
    use state::*;

    macro_rules! state {
        ($turn:expr, [$(($player:expr, $chips:expr)),+]) => {

            {
                let mut assigns = Vec::new();
                $(
                    assigns.push(Assign { player: $player, chips: $chips } );
                )+

                State { turn: $turn, assigns }
            }
        }
    }

    #[derive(Clone, Debug)]
    enum Roll {
        Left,
        Center,
        Right,
        Dot
    }


    macro_rules! roll {
        ($state:expr) => {
            {

                // We test some combinations twice, but doing so has no effect.
                // This makes the code much simpler.
                let mut rolls = Vec::new();
                let mut initial = vec![Roll::Left, Roll::Right, Roll::Center, Roll::Dot];

                for roll1 in initial.iter() {
                    for roll2 in initial.iter() {
                        for roll3 in initial.iter() {
                            rolls.push(vec![roll1.clone(), roll2.clone(), roll3.clone()]);
                        }
                    }
                }

                for outcome in rolls {
                    let old_state = $state.clone();
                    let mut new_state = old_state.clone();
                    new_state.turn += 1;

                    let player = $state.turn as usize;
                    let num_players = $state.assigns.len();

                    let left_player = (if player == 0 { num_players - 1 } else { (player - 1)});
                    let right_player = (if player == num_players - 1 { 0 } else { (player + 1)});

                    let mut prob = 1.0;
                    for roll in outcome {
                        match roll {
                            Roll::Left => {
                                new_state.assigns[player].chips -= 1;
                                new_state.assigns[left_player].chips += 1;
                                prob *= LEFT_PROB;
                            },
                            Roll::Right => {
                                new_state.assigns[player].chips -= 1;
                                new_state.assigns[right_player].chips += 1;
                                prob *= RIGHT_PROB;
                            },
                            Roll::Center => {
                                new_state.assigns[player].chips -= 1;
                                prob *= CENTER_PROB;
                            },
                            Roll::Dot => prob *= DOT_PROB
                        }
                    }

                    println!("Calcing prob:");
                    println!("{:?}", old_state);
                    println!("{:?}", new_state);

                    let computed_prob = transition_prob(&old_state, &new_state, num_players as u8);
                    assert_eq!(computed_prob, prob);

                }

            }
        }
    }

    #[test]
    fn test_() {
        let state1 = state!(0, [(0, 3), (1, 4), (2, 5)]);
        let state2 = state!(1, [(0, 1), (1, 5), (2, 5)]);

        assert_eq!(transition_prob(&state1, &state2, 2), CENTER_PROB * RIGHT_PROB * DOT_PROB);

        println!("Starting roll");
        roll!(state1);
    }
}
