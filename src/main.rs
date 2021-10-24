use std::io;
use crate::marmot_hole::MarmotHole;

mod marmot_hole;

fn main() {

    let hidding_place = MarmotHole{};

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Error reading form stdin");
    let depth = input_string.trim_end().parse::<u32>().expect("Error parsing input");

    if marmot_hole::find_marmot_in(&hidding_place) && marmot_hole::find_marmot(depth) {
        println!("Found marmot!")
    }
    else {
        println!("No marmots found :(");
    }
}

