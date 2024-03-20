use std::error::Error;
pub mod config;
mod response;

use crate::config::Config;
use crate::response::*;

fn mount_url(config: &Config) -> Result<String, Box<dyn Error>> {
    static API_KEY: &'static str = std::env!("API_KEY");

    let url = "http://ws.audioscrobbler.com/2.0/?method=user.gettopartists&period=7day&format=json&user=".to_string();
    let username = &config.username;
    let option_flag = &config.option_flag;
    let mut option_value = "";
    let default_limit = &String::from("5");
    let limit_flag = &String::from("-l");
    let mut limit = "&limit=".to_string();

    match option_flag {
        limit_flag => {
            option_value = match &config.option_value {
                Some(value) => value,
                None  => default_limit
            };
        },
        _ => {},
    };

    limit.push_str(option_value);

    println!("option value is {}", &option_value);

    let result = format!("{}{}{}&api_key={}", url, username, limit, API_KEY);
    dbg!(&result);

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
