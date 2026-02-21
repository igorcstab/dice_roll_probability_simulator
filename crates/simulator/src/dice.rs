pub fn eval_dice_range(number_of_dice: u32, faces: u32) -> Vec<u32> {
    if number_of_dice == 0 || faces == 0 {
        return Vec::new();
    }

    let min = number_of_dice;
    let max = number_of_dice * faces;

    (min..=max).collect()
}
