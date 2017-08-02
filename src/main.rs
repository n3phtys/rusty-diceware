extern crate rand;

mod list;
mod dice;

use rand::OsRng;
use rand::Rng;


fn main() {
    let mut rng = OsRng::new().unwrap();
    let word_list = list::parse_list(list::EN_LIST);
    println!("Hello, world!");
    println!("Random number: {}", rng.next_u32());
    println!("Diceware-PW: {}", build_diceware_password(&mut rng, 5, list));
}

fn build_diceware_password(rng : &mut OsRng, n: u8, list: Vec<&str>) -> String {
    let mut v: Vec<usize> = Vec::new();

    for _ in 0..n {
        v.push(dice::from_array_to_index(dice::roll_four_dice(rng)));
    }

    let mut x: String = "".to_string();

    for (i, item) in v.iter().enumerate() {
        println!("i=={}",i);
        if i > 0usize {
            x = x + "*";
        }
        let idx:usize = *item;
        let r: &str = list[idx];
        x = x + r;
    }

    return x;
}