use std::io::{BufReader, BufRead};
use std::{fs::File};

use reqwest::ClientBuilder;
use reqwest::Client;

pub async fn get_lcu_websocket_connection() {
    let file = File::open("C:\\Riot Games\\League of Legends\\lockfile").unwrap();
    let first_line = BufReader::new(file).lines().next().unwrap().unwrap();
    let content = first_line.split(":");
    let mut content = content.skip(2);
    let port = content.next().unwrap();
    let password = content.next().unwrap();

    let client = ClientBuilder::new().danger_accept_invalid_certs(true).build().unwrap();

    let mut url = "https://127.0.0.1:".to_owned();
    url.push_str(port);
    url.push_str("/lol-summoner/v1/current-summoner");
    
    let response = fire_get_reqest(&client, url, password).await;

    println!("body = {:?}", response);
}


async fn fire_get_reqest(client: &Client, url: String, password: &str) -> String {
    let response = client.get(url).basic_auth("riot", Some(password)).send().await.unwrap();
  
    println!("Response: {:?}", response);
    let text = response.text().await.unwrap();
    println!("Text: {:?}", text);

    text
}
