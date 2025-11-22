#![allow(unused)]

use rand::prelude::*;

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

fn round_to_cent(cash: f64) -> f64 {
    (100. * cash).round() / 100.0
}

#[derive(Debug, Clone)]
struct State {
    heads_chance: f64,
    coin: Coin,
    heads_combo_mult: f64,
    cash: f64,

    histo: [usize; 11],
}

impl State {
    fn new() -> Self {
        Self {
            heads_chance: 0.2,
            coin: Penny,
            heads_combo_mult: 0.5,
            cash: 0.0,

            histo: [0; 11],
        }
    }

    fn combo_reward(&self, streak: u32) -> f64 {
        assert!(streak >= 1);
        assert!(streak <= 10);
        self.coin.dollars() * self.heads_combo_mult.powi(streak as i32 - 1).ceil()
    }

    fn dollars(&self) -> String {
        format!("${:.2}", self.cash)
    }

    fn flip_until_10(&mut self) -> usize {
        let mut flips = 0;
        let mut streak = 0;

        loop {
            // Try to upgrade anything if we can
            if let Some(cost) = self.coin.upgrade_cost()
                && self.cash >= cost
            {
                println!("[{flips:>3}] {}", self.dollars());

                let old = self.coin.clone();
                self.coin.upgrade();
                self.cash -= cost;
                println!("    {old:?} -> {:?}", self.coin);
            }

            assert!(self.cash >= 0.0, "cash={:.2}", self.cash);
            {
                let cash = self.cash;
                let next = self.cash.next_up();
                assert!(
                    next - cash < 0.01,
                    "cash=${:.2} is so much money, we can't track pennies anymore. Give up.",
                    cash
                );
            }

            let next = flip(self);
            flips += 1;

            if next == H {
                streak += 1;

                let gain = self.combo_reward(streak);
                println!("[{flips:>3}] {next:?} {} + ${gain:.2}", self.dollars());
                self.cash += gain;
                self.cash = round_to_cent(self.cash);

                if streak == 10 {
                    self.histo[streak as usize] += 1;
                    break;
                }
            } else {
                // if streak > 0 {
                //     println!("    streak = {streak}");
                // }
                self.histo[streak as usize] += 1;
                streak = 0;
            }
        }
        println!();

        flips
    }
}

fn main() {
    let mut state = State::new();
    state.heads_combo_mult = 1.0 + 0.5;

    let flips = state.flip_until_10();

    println!("Got 10-Heads in {flips} flips:");
    println!("   Odds: {:.2}%", state.heads_chance * 100.);
    println!("   Coin: {:?}", state.coin);
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
    Dollar,
}
use Coin::*;

impl Coin {
    fn dollars(&self) -> f64 {
        match self {
            Penny => 0.01,
            Nickle => 0.05,
            Dime => 0.10,
            Quarter => 0.25,
            Dollar => 1.00,
        }
    }

    fn upgrade_cost(&self) -> Option<f64> {
        match self {
            Penny => Some(0.25),
            Nickle => Some(1.00),
            Dime => Some(6.25),
            Quarter => Some(100.00),
            Dollar => None,
        }
    }

    fn upgrade(&mut self) {
        let next = match self {
            Penny => Nickle,
            Nickle => Dime,
            Dime => Quarter,
            Quarter => Dollar,
            Dollar => unreachable!("Cannot upgrade {self:?}"),
        };
        *self = next;
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn check_round_to_cent() {
        assert_eq!(round_to_cent(0.051234), 0.05);
    }

    #[test]
    fn check_combo_reward_penny() {
        // See: https://docs.google.com/spreadsheets/d/1pYChxjP15Q21vibqacCil9VoOiuoNK1Siuw27SFxVVU/edit?gid=0#gid=0
        let mut state = State::new();
        state.coin = Penny;
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
        state.coin = Dollar;
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
