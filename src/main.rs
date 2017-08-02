extern crate rand;
extern crate clap;

mod list;
mod dice;

use rand::OsRng;
use clap::{Arg, App};


fn main() {
    let matches = App::new("Rusty Diceware")
        .version("0.1")
        .author("Christopher Kaag <christopher.kaag@gmail.com>")
        .about("Generates easy to remember Diceware passwords based on 4d6 roll word lists")
        .arg(Arg::with_name("number")
            .short("n")
            .multiple(false)
            .takes_value(true)
            .help("Sets the number of words to use"))
        .get_matches();


    let n_str: &str = match matches.value_of("number") {
        Some(i) => i,
        None => "5",
    };

    let n: u32 = n_str.parse().unwrap();

    //println!("n = {}", n);



    let mut rng = OsRng::new().unwrap();
    let word_list: Vec<&str> = list::parse_list(list::EN_LIST);
    build_diceware_password(&mut rng, n, &word_list);
}

fn build_diceware_password(rng : &mut OsRng, n: u32, list: &Vec<&str>) -> () {
    let mut v: Vec<usize> = Vec::new();

    for _ in 0..n {
        v.push(dice::from_array_to_index(dice::roll_four_dice(rng)));
    }



    for (i, item) in v.iter().enumerate() {
        if i > 0usize {
            print!("*");
        }
        let idx:usize = *item;
        let r: &str = list[idx];
        print!("{}", r);
    }

}