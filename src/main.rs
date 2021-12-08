use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => solve_day(args[1].trim().parse().expect("You must enter a number between 1 and 25.")),
        _ => invalid_input()
    }
}


fn solve_day(day: i32) {
    if !(1..=25).contains(&day) {
        invalid_input();
    }
    let fn_day = match day {
        1 => day_01::day_01,
        2 => day_02::day_02,
        3 => day_03::day_03,
        4 => day_04::day_04,
        5 => day_05::day_05,
        6 => day_06::day_06,
        7 => day_07::day_07,
        _ => unimplemented!(),
    };
    println!("# Processing Day {} :", day);
    fn_day()
}

fn invalid_input() {
    panic!("You must enter a number between 1 and 25.")
}
