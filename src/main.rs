use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Flip {
    H,
    T,
}
use Flip::*;

fn flip(state: &State) -> Flip {
    let x = rand::rng().random_range(0.0..1.0);
    if x <= state.heads_chance.odds() { H } else { T }
}

fn round_to_cent(cash: f64) -> f64 {
    (100. * cash).round() / 100.0
}

#[derive(Debug, Clone)]
struct State {
    heads_chance: Chance,
    coin: Coin,
    heads_combo_mult: ComboMult,
    cash: f64,
}

struct SimResults {
    flips: usize,
    histo: [usize; 11],
}

impl State {
    fn new() -> Self {
        Self {
            heads_chance: C20,
            coin: Penny,
            heads_combo_mult: ComboMult::Combo1_0x,
            cash: 0.0,
        }
    }

    fn dollars(&self) -> String {
        format!("${:.2}", self.cash)
    }

    fn flip_until_10(&mut self) -> SimResults {
        let mut flips = 0;
        let mut streak = 0;
        let mut histo = [0_usize; 11];

        loop {
            // Try to upgrade anything if we can
            self.try_upgrade(flips);

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

            // On heads, gain money
            if next == H {
                streak += 1;

                let gain = self.coin.dollars() * self.heads_combo_mult.mult(streak);
                self.cash += gain;
                self.cash = round_to_cent(self.cash);
            }

            // Handle ending a streak
            if next == T || streak == 10 {
                if histo[streak as usize] == 0 {
                    println!(
                        "[{flips:>5}] {cash:>10} Streak! {streak}",
                        cash = self.dollars()
                    );
                }
                histo[streak as usize] += 1;

                if streak == 10 {
                    break;
                }
                streak = 0;
            }
        }
        println!();

        SimResults { flips, histo }
    }

    fn try_upgrade(&mut self, flips: usize) {
        // TODO: Take a thing to upgrade instead.

        // Coin
        if let Some(cost) = self.coin.upgrade_cost()
            && self.cash >= cost
        {
            let cash = self.dollars();

            let old = self.coin.clone();
            self.coin.upgrade();
            self.cash -= cost;
            println!("[{flips:>5}] {cash:>10} ++ {old:?} -> {:?}", self.coin);
        }

        // Heads Chance
        if let Some(cost) = self.heads_chance.upgrade_cost()
            && self.cash >= cost
        {
            let cash = self.dollars();

            let old = self.heads_chance.clone();
            self.heads_chance.upgrade();
            self.cash -= cost;
            println!(
                "[{flips:>5}] {cash:>10} ++ {old:?} -> {:?}",
                self.heads_chance
            );
        }

        // Combo Mult
        if let Some(cost) = self.heads_combo_mult.upgrade_cost()
            && self.cash >= cost
        {
            let cash = self.dollars();

            let old = self.heads_combo_mult.clone();
            self.heads_combo_mult.upgrade();
            self.cash -= cost;
            println!(
                "[{flips:>5}] {cash:>10} ++ {old:?} -> {:?}",
                self.heads_combo_mult
            );
        }
    }
}

fn main() {
    // do it twice
    for _ in 0..2 {
        println!("################################");
        let mut state = State::new();
        let SimResults { flips, histo } = state.flip_until_10();

        println!("Got 10-Heads in {flips} flips:");
        println!("   Odds:  {:.0}%", state.heads_chance.odds() * 100.);
        println!("   Combo: {:?}", state.heads_combo_mult);
        println!("   Coin:  {:?}", state.coin);
        for (i, &count) in histo.iter().enumerate() {
            if count == 0 {
                continue;
            }
            if i > 0 {
                println!("    {i:>2}-run: {count} time(s)")
            } else {
                println!("     tails: {count} time(s)");
            }
        }
        println!("################################");
        println!();
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Chance {
    C20,
    C25,
    C30,
    C35,
    C40,
    C45,
    C50,
    C55,
    C60, // Allegedly??
}
use Chance::*;

impl Chance {
    fn odds(&self) -> f64 {
        match self {
            C20 => 0.20,
            C25 => 0.25,
            C30 => 0.30,
            C35 => 0.35,
            C40 => 0.40,
            C45 => 0.45,
            C50 => 0.50,
            C55 => 0.55,
            C60 => 0.60,
        }
    }

    fn upgrade_cost(&self) -> Option<f64> {
        match self {
            C20 => Some(1e-2),
            C25 => Some(1e-1),
            C30 => Some(1e0),
            C35 => Some(1e1),
            C40 => Some(1e2),
            C45 => Some(1e3),
            C50 => Some(1e4),
            C55 => Some(1e5),
            C60 => None,
        }
    }

    fn upgrade(&mut self) {
        let next = match self {
            C20 => C25,
            C25 => C30,
            C30 => C35,
            C35 => C40,
            C40 => C45,
            C45 => C50,
            C50 => C55,
            C55 => C60,
            C60 => unreachable!("Cannot upgrade {self:?}"),
        };
        *self = next;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComboMult {
    Combo1_0x,
    Combo1_5x,
    Combo2_0x,
    Combo2_5x,
    Combo3_0x,
    Combo3_5x,
}
use ComboMult::*;

impl ComboMult {
    fn mult(&self, streak: i32) -> f64 {
        let base: f64 = match self {
            Combo1_0x => 1.0,
            Combo1_5x => 1.5,
            Combo2_0x => 2.0,
            Combo2_5x => 2.5,
            Combo3_0x => 3.0,
            Combo3_5x => 3.5,
        };

        assert!(streak >= 1);
        assert!(streak <= 10);
        base.powi(streak - 1).ceil()
    }

    fn upgrade_cost(&self) -> Option<f64> {
        match self {
            Combo1_0x => Some(1e0),
            Combo1_5x => Some(1e1),
            Combo2_0x => Some(1e2),
            Combo2_5x => Some(1e3),
            Combo3_0x => Some(1e4),
            Combo3_5x => None,
        }
    }

    fn upgrade(&mut self) {
        let next = match self {
            Combo1_0x => Combo1_5x,
            Combo1_5x => Combo2_0x,
            Combo2_0x => Combo2_5x,
            Combo2_5x => Combo3_0x,
            Combo3_0x => Combo3_5x,
            Combo3_5x => unreachable!("Cannot upgrade {self:?}"),
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

        let rewards: Vec<_> = (1..10)
            .map(|i| Penny.dollars() * Combo1_5x.mult(i))
            .collect();
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

        let rewards: Vec<_> = (1..10)
            .map(|i| Dollar.dollars() * Combo3_5x.mult(i))
            .collect();
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
