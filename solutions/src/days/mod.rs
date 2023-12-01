use common::Solution;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

pub fn get_day(day: u8) -> Result<Box<dyn Solution>, String> {
    match day {
        1 => Ok(Box::<day_01::Puzzle>::default()),
        2 => Ok(Box::<day_02::Puzzle>::default()),
        3 => Ok(Box::<day_03::Puzzle>::default()),
        4 => Ok(Box::<day_04::Puzzle>::default()),
        5 => Ok(Box::<day_05::Puzzle>::default()),
        6 => Ok(Box::<day_06::Puzzle>::default()),
        7 => Ok(Box::<day_07::Puzzle>::default()),
        8 => Ok(Box::<day_08::Puzzle>::default()),
        9 => Ok(Box::<day_09::Puzzle>::default()),
        10 => Ok(Box::<day_10::Puzzle>::default()),
        11 => Ok(Box::<day_11::Puzzle>::default()),
        12 => Ok(Box::<day_12::Puzzle>::default()),
        13 => Ok(Box::<day_13::Puzzle>::default()),
        14 => Ok(Box::<day_14::Puzzle>::default()),
        15 => Ok(Box::<day_15::Puzzle>::default()),
        16 => Ok(Box::<day_16::Puzzle>::default()),
        17 => Ok(Box::<day_17::Puzzle>::default()),
        18 => Ok(Box::<day_18::Puzzle>::default()),
        19 => Ok(Box::<day_19::Puzzle>::default()),
        20 => Ok(Box::<day_20::Puzzle>::default()),
        21 => Ok(Box::<day_21::Puzzle>::default()),
        22 => Ok(Box::<day_22::Puzzle>::default()),
        23 => Ok(Box::<day_23::Puzzle>::default()),
        24 => Ok(Box::<day_24::Puzzle>::default()),
        25 => Ok(Box::<day_25::Puzzle>::default()),
        _ => Err(String::from("Day not yet created")),
    }
}
