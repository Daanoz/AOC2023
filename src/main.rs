use clap::ArgAction;
use clap::Parser;
use std::env;
use std::fs;
use std::ops::Add;
use std::path;
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
    all: bool
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.day.is_none() && !args.all {
        panic!("Either select a day with --day <DAY> or run all with --all");
    }
    if let Some(day) = args.day {
        execute_day(day, args.aoc_session.clone()).await;
    } else {
        let mut total_duration = Duration::ZERO;
        for day in 1..=25 {
            total_duration = total_duration.add(execute_day(day, args.aoc_session.clone()).await);
        }
        println!("Total time: {:.2?}", total_duration);
    }

}

async fn execute_day(day: u8, aoc_session: Option<String>) -> Duration {
    let mut solution = match days::get_day(day) {
        Ok(solution) => solution,
        Err(_) => return Duration::ZERO
    };
    
    let input = get_input(day, aoc_session).await.unwrap();
    
    let start = std::time::Instant::now();

    let result_a = solution.solve_a(input.clone()).await;
    let time_a = start.elapsed();
    let start_b = std::time::Instant::now();
    let result_b = solution.solve_b(input.clone()).await;
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

    println!("Day {} time: {:.2?} (A: {:.2?}, B: {:.2?})", day, time, time_a, time_b);
    time
}

async fn get_input(day: u8, aoc_session: Option<String>) -> Result<String, String> {
    let filename = format!("input_{}.txt", day);
    let input_path = path::Path::new(&env::current_dir().map_err(|e| e.to_string())?)
        .join("inputs")
        .join(&filename);
    let aoc_session = aoc_session.or(env::var_os("AOC_SESSION").and_then(|v| v.into_string().ok()));

    if input_path.is_dir() {
        panic!("Input file is a directory!")
    }
    if input_path.exists() && input_path.is_file() {
        if let Ok(data) = fs::read_to_string(&input_path) {
            return Ok(data);
        }
    }
    fs::create_dir_all(input_path.parent().unwrap()).map_err(|e| e.to_string())?;

    let aoc_session = if let Some(session_id) = aoc_session {
        session_id
    } else {
        return Err("Cannot download input, AOC_SESSION unavailable".to_string());
    };

    let url = format!("https://adventofcode.com/{}/day/{}/input", "2023", day);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("cookie", format!("session={}", aoc_session))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status();
    let text: String = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("Downloading input failed: {}; {}", status, text));
    }
    fs::write(input_path, text.trim_end()).map_err(|e| e.to_string())?;
    Ok(text)
}
