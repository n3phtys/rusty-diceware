extern crate rand;

mod list;
mod dice;

use rand::OsRng;


fn main() {
    let mut rng = OsRng::new().unwrap();
    let word_list: Vec<&str> = list::parse_list(list::EN_LIST);
    println!("{}", build_diceware_password(&mut rng, 5, &word_list));
}

fn build_diceware_password(rng : &mut OsRng, n: u8, list: &Vec<&str>) -> String {
    let mut v: Vec<usize> = Vec::new();

    for _ in 0..n {
        v.push(dice::from_array_to_index(dice::roll_four_dice(rng)));
    }

    let mut x: String = "".to_string();

    for (i, item) in v.iter().enumerate() {
        if i > 0usize {
            x = x + "*";
        }
        let idx:usize = *item;
        let r: &str = list[idx];
        x = x + r;
    }

    return x;
}