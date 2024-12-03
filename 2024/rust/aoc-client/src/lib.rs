use std::{env, path::PathBuf};
use tokio::fs;

#[tracing::instrument]
pub async fn get_input(mut root: PathBuf, day: u8) -> anyhow::Result<String> {
    root.push(format!("day-{day}/inputs/input.txt"));
    let filename = root.as_path();
    match fs::read_to_string(&filename).await {
        Ok(input) if !input.is_empty() => Ok(input),
        _ => {
            let session_token = env::var("SESSION_TOKEN")?;
            let client = reqwest::Client::new();
            let input = client
                .get(format!("https://adventofcode.com/2024/day/{day}/input"))
                .header("Cookie", format!("session={session_token}"))
                .send()
                .await?
                .text()
                .await?;
            fs::write(filename, &input).await?;
            Ok(input)
        }
    }
}

#[tracing::instrument]
pub fn block_on_input(day: u8) -> String {
    let root = std::fs::canonicalize("..").expect("Parent dir");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { get_input(root, day).await.expect("Get input") })
}
