use std::collections::HashMap;

use reqwest::{self};
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
	ArgGroup::new("arg")
		.required(true)
		.args(["email"])
))]

struct Args {
	email: String
}

static URL: &str = "https://bitmoji.api.snapchat.com/api/user/find";

#[tokio::main]
async fn main() {
	let email = Args::parse().email;
	let client = reqwest::Client::new();
	let mut map = HashMap::new();
	map.insert("email", email);
	let response = client.post(URL).json(&map).send().await.unwrap();
	let content = response.text().await.unwrap();
	let value = content.split_at(content.find(":").unwrap_or(0)).1;
	if value.is_empty() {
		println!("Not used");
	}
	else {
		println!("Mail used on {}", value.replace(&[':', '"', '}'][..], ""));
	}
}
