use aoc2023::get_input;
use chrono::{ TimeZone, Datelike, Local};
use clap::ArgAction;
use clap::Parser;
use std::ops::Add;
use std::time::Duration;

mod days;

#[derive(Parser, Debug)]
#[command(name = "AOC 2023")]
#[command(author = "Daan Sieben")]
#[command(version = "1.0")]
#[command(about, long_about = None)]
struct Args {
    /// AOC Session id; if not set uses ENV var AOC_SESSION
    #[arg(long)]
    aoc_session: Option<String>,
    /// Puzzle day to run
    #[arg(short, long)]
    day: Option<u8>,
    // Run all days
    #[arg(short, long, action = ArgAction::SetTrue)]
    all: bool,
}

#[tokio::main]
async fn main() {


    std::env::vars().for_each(|(key, value)| {
        println!("{}: {}", key, value);
    });

    let args = Args::parse();
    if args.day.is_none() && !args.all {
        panic!("Either select a day with --day <DAY> or run all with --all");
    }
    let now = Local::now();
    let max_day = if now.year() == 2023 && now.month() == 12 {
        now.day() as u8
    } else if now.lt(&Local.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).earliest().unwrap()) {
        0
    } else {
        25
    };

    if let Some(day) = args.day {
        if day > max_day {
            panic!("Day {} is not yet available", day);
        }
        execute_day(day, args.aoc_session.clone()).await;
    } else {
        let mut total_duration = Duration::ZERO;
        for day in 1..=25 {
            if day <= max_day {
                total_duration = total_duration.add(execute_day(day, args.aoc_session.clone()).await);
            } else {
                println!("Day {} skipped", day);
            }
        }
        println!("Total time: {:.2?}", total_duration);
    }
}

async fn execute_day(day: u8, aoc_session: Option<String>) -> Duration {
    let mut solution = match days::get_day(day) {
        Ok(solution) => solution,
        Err(_) => return Duration::ZERO,
    };

    let input_a = get_input(day, aoc_session).await.unwrap();
    let input_b = input_a.clone();

    let start = std::time::Instant::now();
    let result_a = solution.solve_a(input_a);
    let time_a = start.elapsed();
    let start_b = std::time::Instant::now();
    let result_b = solution.solve_b(input_b);
    let time_b = start_b.elapsed();
    let time = start.elapsed();

    match result_a {
        Ok(answer) => println!("Day {}, Part A = {}", day, answer.get_result()),
        Err(error) => println!("Day {}, Part A failed! {}", day, error),
    }
    match result_b {
        Ok(answer) => println!("Day {}, Part B = {}", day, answer.get_result()),
        Err(error) => println!("Day {}, Part B failed! {}", day, error),
    }

    println!(
        "Day {} time: {:.2?} (A: {:.2?}, B: {:.2?})",
        day, time, time_a, time_b
    );
    time
}
