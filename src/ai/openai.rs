use reqwest::Client;
use serde::{Deserialize, Serialize};
// use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

pub async fn get_ai_suggestion(
    client: &Client,
    prompt: &str,
    api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let request_body = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        max_tokens: 150,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        println!("Error: Received non-success status code {}", status);
        return Err(format!("Non-success status code: {}", status).into());
    }

    let openai_response: OpenAIResponse = serde_json::from_str(&text)?;
    let choice = openai_response
        .choices
        .first()
        .ok_or("No choices in response")?;
    Ok(choice.message.content.clone())
}
