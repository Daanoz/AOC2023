use std::{path, env, fs};

pub mod days;

pub async fn get_input(day: u8, aoc_session: Option<String>) -> Result<String, String> {
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