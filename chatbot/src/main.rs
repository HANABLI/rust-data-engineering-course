/**
 * This project uses several libraries:
 * - reqwest for making HTTP requests,
 * - serde_json for handling JSON data,
 * - std::io for handling input/output.
 * 
 * It also uses the OpenAI API to interact with an AI model.
 * 
 * The main function, run_chat_loop, runs a chat loop that:
 * - Reads user input,
 * - Appends it to the conversation,
 * - Sends the conversation to the OpenIA API,
 * - Reseives the model' response and then prints the response in the prompt
 * - repeat thios process until the user types exit or quit.
 * 
 */

 use chatbot::chatbot::run_chat_loop;
 use reqwest::Client;
 use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    // Use env variable OPENAI_API_KEY
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY should be set");
    let url = "https://api.openai.com/v1/completions";

    run_chat_loop(&client, &api_key, url).await?;

    Ok(())
}
