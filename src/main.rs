use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env; // env module for env variables, OpenAi access key
use std::io::{stdin, stdout, Write};

// A struct to work with the API response
#[derive(Debug, Deserialize)]
struct OAIResponse {
  id: Option<String>,
  object: Option<String>,
  created: Option<u64>,
  model: Option<String>,
  choices: Vec<OAIChoices>,
}

// A struct for the choices
#[derive(Debug, Deserialize)]
struct OAIChoices {
  text: String,
  index: u8,
  logprobs: Option<u8>,
  finish_reason: String,
}

//A struct for the OAI request
#[derive(Debug, Serialize)]
struct OAIRequest {
  prompt: String,
  max_tokens: u16,
}

// Tokio async main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  // Load .env file
  dotenv().ok();
  // Create a Httpconnector
  let https = HttpsConnector::new();
  // Create a Client
  let client = Client::builder().build(https); //client from hyper crate
  let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions"; // OpenAI API endpoint
  let preamble = "Generate a Sql code for the given statement."; // preamble for the prompt
                                                                 //Token in the header
  let oai_token: String = env::var("OAI_TOKEN").unwrap();
  let auth_header_val = format!("Bearer {}", oai_token);
  println!("{esc}c", esc = 27 as char);

  loop {
    print!(">");
    stdout().flush().unwrap();
    let mut user_text = String::new();

    stdin()
      .read_line(&mut user_text)
      .expect("Failed to read line");
    println!("");
    // Spinner, wait for the response
    let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is thinking...".into());
    // Request to chatGPT for every single user input
    let oai_request = OAIRequest {
      prompt: format!("{} {}", preamble, user_text),
      max_tokens: 1000,
    };
    let body = Body::from(serde_json::to_vec(&oai_request)?);
    let req = Request::post(uri)
      .header(header::CONTENT_TYPE, "application/json")
      .header("Authorization", &auth_header_val)
      .body(body)
      .unwrap();
    // Response and print
    let res = client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let json: OAIResponse = serde_json::from_reader(body.reader())?;
    sp.stop();
    println!("");
    println!("{}", json.choices[0].text);
  }
}
