use crate::aoc_2024::{day_1::Day1, day_2::Day2};

pub trait AoCResult {
    fn part_1(&self);
    fn part_2(&self);
}

#[derive(Default)]
pub struct Resolve {}

impl Resolve {
    pub fn day() -> Box<dyn AoCResult> {
        match get_day() {
            "1" => Box::new(Day1),
            "2" => Box::new(Day2),
            _ => todo!("Pending day!"),
        }
    }

    pub fn done() {
        let day = Resolve::day();
        match get_part() {
            "1" => day.part_1(),
            "2" => day.part_2(),
            _ => todo!("Pending part!"),
        }
    }
}

pub fn get_day<'a>() -> &'a str {
    option_env!("DAY").unwrap_or("1")
}
pub fn get_part<'a>() -> &'a str {
    option_env!("PART").unwrap_or("1")
}
pub fn get_input(day: u8) -> String {
    format!("./inputs/2024/day_{day}.txt")
}

pub mod fn_utils {
    use core::panic;
    use std::{fs::File, io::Read};

    pub fn input(path: &str) -> String {
        let mut contents = String::new();
        match File::open(path) {
            Ok(mut file) => match file.read_to_string(&mut contents) {
                Ok(_res) => contents,
                Err(e) => panic!("Internal error {}", e),
            },
            Err(e) => panic!("Internal error {}", e),
        }
    }
}
