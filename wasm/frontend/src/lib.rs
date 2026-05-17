use simulator::comparison::Operation;
use simulator::dice::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_range(number_of_dice: u32, faces: u32) -> Vec<u32> {
    eval_dice_range(number_of_dice, faces)
}

#[wasm_bindgen]
pub fn run_simulation(
    number_of_rolls: u32,
    number_of_dice: u32,
    faces: u32,
    operation: &str,
    target: u32,
) -> f64 {
    let operation = match operation {
        "greater_or_equal" => Operation::GreaterOrEqual,
        "greater" => Operation::Greater,
        "equal" => Operation::Equal,
        "smaller_or_equal" => Operation::SmallerOrEqual,
        "smaller" => Operation::Smaller,

        _ => panic!("invalid operation"),
    };

    let simulation = Simulation {
        number_of_rolls,

        dice: Dice {
            number_of_dice,
            faces,
        },

        condition: Condition { operation, target },
    };

    let mut rng = rand::thread_rng();

    simulation.run(&mut rng)
}
