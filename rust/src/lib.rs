use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
//mod day7;
//mod day8;
//mod day9;

//mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

pub type Res<T> = Result<T, Box<dyn Error>>;

pub fn solution(day: u32) -> Res<()> {
    assert!(1 <= day && day <= 25);
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        //7 => day7::main(),
        //8 => day8::main(),
        //9 => day9::main(),
        //10 => day10::main(),
        11 => day11::main(),
        12 => day12::main(),
        13 => day13::main(),
        14 => day14::main(),
        15 => day15::main(),
        //16 => day16::main(),
        //17 => day17::main(),
        //18 => day18::main(),
        //19 => day19::main(),
        //20 => day20::main(),
        //21 => day21::main(),
        //22 => day22::main(),
        //23 => day23::main(),
        //24 => day24::main(),
        //25 => day25::main(),
        _ => Err(format!("Not yet implemented: day {}", day).into()),
    }
}
