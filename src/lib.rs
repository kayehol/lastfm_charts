use std::error::Error;
use std::env;
pub mod config;
mod response;

use crate::config::Config;
use crate::response::*;

fn mount_url(config: &Config) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("API_KEY")?;
    let url = "http://ws.audioscrobbler.com/2.0/?method=user.gettopartists&period=7day&limit=5&format=json&user=".to_string();

    let result = format!("{}{}&api_key={}", url, &config.username, api_key);

    Ok(result)
}

fn get_artists(url: &str) -> Result<Response, reqwest::Error> {
    let result: Response = reqwest::blocking::get(url)?.json()?;
    Ok(result)
}

fn format(response: &Response) {
    let artists = &response.topartists.artist;
    let username = &response.topartists.attr.user;

    println!("{}'s weekly charts", username);
    for artist in artists {
        let rank = &artist.attr.rank;
        let name = &artist.name;

        println!("{}. {}", rank, name);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let url = mount_url(&config)?;
    let response = get_artists(&url)?;
    format(&response);

    Ok(())
}
