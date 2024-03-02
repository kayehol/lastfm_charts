use std::error::Error;
use std::env;
pub mod config;
mod response;

use crate::config::Config;
use crate::response::*;

fn mount_url(config: &Config) -> String {
    let api_key = env::var("API_KEY").unwrap();
    let mut url = "http://ws.audioscrobbler.com/2.0/?method=user.gettopartists&period=7day&limit=5&format=json&user=".to_string();
    url.push_str(&config.username);
    url.push_str("&api_key=");
    url.push_str(&api_key);

    url 
}

fn get_artists(url: &str) -> Result<Response, reqwest::Error> {
    let result: Response = reqwest::blocking::get(url)?.json()?;
    Ok(result)
}

fn format(response: &Response) {
    let artists = &response.topartists.artist;
    for artist in artists {
        let rank = &artist.attr.rank;
        println!("{}. {}", rank, artist.name);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let url = mount_url(&config);
    let response = get_artists(&url)?;
    format(&response);

    Ok(())
}
