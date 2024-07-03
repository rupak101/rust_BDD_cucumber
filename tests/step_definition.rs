use cucumber::{ given, when, then, World };
use reqwest::{ Client, Response };
use std::env;
use dotenv::dotenv;
use serde_json::Value;

// Import the functions from the lib.rs file
use tech_challenge::{ generate_signature, get_nonce };

#[derive(Debug, Default)]
struct ApiDetails {
    api_url: String,
    api_key: String,
    api_secret: String,
    response: Option<Response>,
}

impl ApiDetails {
    fn new() -> Self {
        Self {
            api_url: String::new(),
            api_key: String::new(),
            api_secret: String::new(),
            response: None,
        }
    }
}

// `World` is shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct BaseUrl {
    api_details: ApiDetails,
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("user set base url for retriving the server time")]
fn set_base_url(world: &mut BaseUrl) {
    dotenv().ok();
    let api_url = env::var("BASE_URL").expect("API_ENDPOINT must be set in .env file");
    world.api_details.api_url = format!("{}public/Time", api_url);
}

#[when("user retrievs the server time details")]
async fn retrive_server_time(world: &mut BaseUrl) {
    let response = reqwest::get(&world.api_details.api_url).await.unwrap();
    world.api_details.response = Some(response);
}

#[then("the response should be successful")]
fn validate_status_code(world: &mut BaseUrl) {
    let response = world.api_details.response.as_ref().unwrap();
    assert_eq!(response.status(), 200);
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("user set base url for trading pair")]
fn set_base_url_trading(world: &mut BaseUrl) {
    dotenv().ok();
    let api_url = env::var("BASE_URL").expect("API_ENDPOINT must be set in .env file");
    world.api_details.api_url = format!("{}public/Depth?", api_url);
}

#[when(expr = "user retrievs the trading pair {} information")]
async fn retrive_trading_pair(world: &mut BaseUrl, pair: String) {
    world.api_details.api_url = format!("{}pair={}", world.api_details.api_url, pair);
    let response = reqwest::get(&world.api_details.api_url).await.unwrap();
    world.api_details.response = Some(response);
}

#[then("the response should contain the trading pair details")]
async fn validate_trading_pair_details(
    world: &mut BaseUrl
) -> Result<i64, Box<dyn std::error::Error>> {
    let url = world.api_details.api_url.clone();
    let client = Client::new();
    let response: Value = client.get(&url).send().await?.json().await?;

    // Check for errors in the response
    if !response["error"].as_array().unwrap().is_empty() {
        return Err(format!("API error: {:?}", response["error"]).into());
    }

    // Extract and return the trading pair
    let tradingpair = response["result"]
        .as_i64()
        .ok_or("Failed to parse trading pair details from response")?;
    Ok(tradingpair)
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("user set base url for an account")]
fn set_base_url_for_account(world: &mut BaseUrl) {
    dotenv().ok();
    let api_url = env::var("BASE_URL").expect("API_ENDPOINT must be set in .env file");
    world.api_details.api_url = format!("{}private/OpenOrders", api_url);
    world.api_details.api_key = env::var("API_KEY").expect("API_KEY must be set in .env file");
    world.api_details.api_secret = env
        ::var("API_SECRET")
        .expect("API_SECRET must be set in .env file");
}

#[when("user request the open orders")]
async fn when_request_open_orders(world: &mut BaseUrl) {
    let nonce = get_nonce();
    let post_data = format!("nonce={}", nonce);

    let signature = generate_signature(
        &world.api_details.api_url,
        nonce,
        &post_data,
        &world.api_details.api_secret
    );

    let client = Client::new();
    client
        .post(&world.api_details.api_url)
        .header("API-Key", &world.api_details.api_key)
        .header("API-Sign", signature)
        .body(post_data)
        .send().await
        .expect("Failed to send request")
        .json::<Value>().await
        .expect("Failed to parse response");
}

// This runs before everything else, so we can setup things here.
#[tokio::main]
async fn main() {
    BaseUrl::run("tests/features").await;
}
