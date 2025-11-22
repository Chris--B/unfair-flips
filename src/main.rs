#![allow(unused)]

use rand::prelude::*;

#[derive(Debug, Clone)]
struct State {
    heads_chance: f64,
    base_coin_worth: f64,
    heads_combo_mult: f64,
    cash: f64,

    histo: [usize; 11],
}

impl State {
    fn new() -> Self {
        Self {
            heads_chance: 0.2,
            base_coin_worth: 0.01,
            heads_combo_mult: 0.5,
            cash: 0.0,

            histo: [0; 11],
        }
    }

    fn combo_reward(&self, streak: u32) -> f64 {
        assert!(streak >= 1);
        assert!(streak <= 10);
        self.base_coin_worth * self.heads_combo_mult.powi(streak as i32 - 1).ceil()
    }

    fn dollars(&self) -> String {
        format!("${:.2}", self.cash)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Flip {
    H,
    T,
}
use Flip::*;

fn flip(state: &State) -> Flip {
    let x = rand::rng().random_range(0.0..1.0);
    if x <= state.heads_chance { H } else { T }
}

fn flip_until_10(state: &mut State) -> usize {
    let mut flips = 0;
    let mut streak = 0;

    loop {
        let next = flip(state);
        flips += 1;

        if next == H {
            streak += 1;

            let gain = state.combo_reward(streak);
            println!("[{flips:>3}] {next:?} {} + ${gain:.2}", state.dollars());
            state.cash += gain;
            state.cash = (100. * state.cash).round() / 100.0;

            if streak == 10 {
                state.histo[streak as usize] += 1;
                break;
            }
        } else {
            if streak > 0 {
                println!("    streak = {streak}");
            }
            state.histo[streak as usize] += 1;
            streak = 0;
        }
    }
    println!();

    flips
}

fn main() {
    let mut state = State::new();
    state.heads_chance = 0.8;
    state.base_coin_worth = 0.05;
    state.heads_combo_mult = 1.0 + 0.5;

    let flips = flip_until_10(&mut state);

    println!("Got 10-Heads in {flips} flips:");
    for (i, &count) in state.histo.iter().enumerate() {
        if count == 0 {
            continue;
        }
        if i > 0 {
            println!("    {i:>2}-run: {count} time(s)")
        } else {
            println!("     tails: {count} time(s)");
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn check_combo_reward_penny() {
        // See: https://docs.google.com/spreadsheets/d/1pYChxjP15Q21vibqacCil9VoOiuoNK1Siuw27SFxVVU/edit?gid=0#gid=0
        let mut state = State::new();
        state.base_coin_worth = 0.01;
        state.heads_combo_mult = 1.5;

        let rewards: Vec<_> = (1..10).map(|i| state.combo_reward(i)).collect();
        let expected = [
            0.01, //
            0.02, //
            0.03, //
            0.04, //
            0.06, //
            0.08, //
            0.12, //
            0.18, //
            0.26, //
        ];

        assert_eq!(rewards, expected);
    }

    #[test]
    fn check_combo_reward_dollar() {
        // See: https://docs.google.com/spreadsheets/d/1pYChxjP15Q21vibqacCil9VoOiuoNK1Siuw27SFxVVU/edit?gid=0#gid=0
        let mut state = State::new();
        state.base_coin_worth = 1.00;
        state.heads_combo_mult = 3.5;

        let rewards: Vec<_> = (1..10).map(|i| state.combo_reward(i)).collect();
        let expected = [
            1.00,     //
            4.00,     //
            13.00,    //
            43.00,    //
            151.00,   //
            526.00,   //
            1839.00,  //
            6434.00,  //
            22519.00, //
        ];

        assert_eq!(rewards, expected);
    }
}
