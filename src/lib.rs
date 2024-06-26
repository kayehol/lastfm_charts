use std::error::Error;
pub mod config;
mod response;

use crate::config::Config;
use crate::response::*;

fn mount_url(config: &Config) -> Result<String, Box<dyn Error>> {
    static API_KEY: &str = std::env!("API_KEY");
    let username = &config.username;
    let limit = &config.limit;
    let period = &config.period;

    let url_root = "http://ws.audioscrobbler.com/2.0/?".to_string();
    let method = "&method=user.gettopartists".to_string();
    let format = "&format=json".to_string();

    let mut user_param = "&user=".to_string();
    let mut limit_param = "&limit=".to_string();
    let mut period_param = "&period=".to_string();

    user_param.push_str(username);
    limit_param.push_str(limit);
    period_param.push_str(period);

    let result = format!(
        "{url_root}{method}{period_param}{format}{user_param}{limit_param}&api_key={API_KEY}"
    );

    Ok(result)
}

fn get_artists(url: &str) -> Result<Response, reqwest::Error> {
    let result: Response = reqwest::blocking::get(url)?.json()?;
    Ok(result)
}

fn format(response: &Response) {
    let artists = &response.topartists.artist;
    let username = &response.topartists.attr.user;

    println!("{}'s charts", username);
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
