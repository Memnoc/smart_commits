mod ai;
mod git;

use ai::openai::get_ai_suggestion;
use git::diff::get_last_commit_diff;

use git2::Repository;
use reqwest::Client;
// use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let openai_key =
        env::var("OPENAI_API_KEY").expect("Expected a token in the environment for OpenAI API");

    let client = Client::new();
    let repo = Repository::discover(".")
        .expect("Could not find a repository in this location or any parent directory");

    let commit_message = suggest_commit_message(&client, &repo, &openai_key).await?;
    println!("AI commit: {}", commit_message);

    Ok(())
}

async fn suggest_commit_message(
    client: &Client,
    repo: &Repository,
    openai_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let diff = get_last_commit_diff(repo)?;

    let prompt = format!("Suggest a succinct but descriptive and consistent commit message for the following changes:\n\n{}", diff);
    let suggestion = get_ai_suggestion(client, &prompt, openai_key).await?;

    Ok(suggestion)
}
