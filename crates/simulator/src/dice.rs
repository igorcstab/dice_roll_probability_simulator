use rand::RngExt;

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

pub fn roll_dice(rng: &mut impl rand::Rng, number_of_dice: u32, faces: u32) -> Vec<u32> {
    let mut result = Vec::new();

    for _ in 0..number_of_dice {
        result.push(roll(rng, faces));
    }

    result
}
