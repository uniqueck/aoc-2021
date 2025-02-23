use mr_kaffee_2021_08::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {
    // solve part 1
    let instant_1 = Instant::now();
    let data = parse(&read_input());
    let sol_1 = solution_1(&data);
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1);
    assert_eq!(397, sol_1);

    // solve part 2
    let instant_2 = Instant::now();
    let sol_2 = solution_2(&data);
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2);
    assert_eq!(1_027_422, sol_2);
}
