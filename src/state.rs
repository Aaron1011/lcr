#[derive(Debug, Clone)]
pub struct Assign {
    pub player: u8,
    pub chips: u8
}

#[derive(Debug, Clone)]
pub struct State {
    pub turn: u8,
    pub assigns: Vec<Assign>
}

pub fn create_states(players: &[u8]) -> Vec<State> {
    let mut states = Vec::new();
    let chips =  3 * players.len();

    for num_chips in 1..=chips {
        let permuts = permute(players, num_chips as u8);
        for permute in permuts {
            for turn in players {
                states.push(State { turn: *turn, assigns: permute.clone() });
            }
        }
    }

    states

}

pub fn permute(players: &[u8], chips: u8) -> Vec<Vec<Assign>> {
    let mut result = Vec::new();
    let player = players[0];

    if players.len() == 1 {
        return vec![vec![Assign { player, chips } ]]
    }

    for num_chips in 0..=chips {
        let other_permuts = permute(&players[1..], chips - num_chips);
        for mut permut in other_permuts {
            permut.push(Assign { player, chips: num_chips });
            result.push(permut);
        }
    }

    return result
}
