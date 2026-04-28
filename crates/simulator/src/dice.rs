use rand::RngExt;

use crate::comparison::*;

pub struct Dice {
    pub number_of_dice: u32,
    pub faces: u32,
}

pub struct Condition {
    pub operation: Operation,
    pub target: u32,
}

pub struct Simulation {
    pub number_of_rolls: u32,
    pub dice: Dice,
    pub condition: Condition,
}

pub fn eval_dice_range(number_of_dice: u32, faces: u32) -> Vec<u32> {
    if number_of_dice == 0 || faces == 0 {
        return Vec::new();
    }

    let min = number_of_dice;
    let max = number_of_dice * faces;

    (min..=max).collect()
}

pub fn roll(rng: &mut impl rand::Rng, faces: u32) -> u32 {
    rng.random_range(1..=faces)
}

impl Condition {
    pub fn check(&self, value: u32) -> bool {
        compare(value as i32, self.operation, self.target as i32)
    }
}

impl Simulation {
    pub fn run(&self, rng: &mut impl rand::Rng) -> f64 {
        if self.number_of_rolls == 0 {
            return 0.0;
        }

        let success = (0..self.number_of_rolls)
            .filter(|_| {
                let sum: u32 = (0..self.dice.number_of_dice)
                    .map(|_| rng.random_range(1..=self.dice.faces))
                    .sum();

                self.condition.check(sum)
            })
            .count();

        100.0 * success as f64 / self.number_of_rolls as f64
    }
}
