use clap::Parser;

pub struct Config {
    pub username: String,
    pub limit: String,
    pub period: String,
}

#[derive(Parser)]
#[command(name = "LastFM Charts")]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    username: String,
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=10))]
    limit: Option<u8>,
    #[arg(short, long, value_parser = ["7day", "1month", "6month", "12month", "overall"])]
    period: Option<String>,
}

impl Config {
    pub fn build() -> Config {
        let cli = Cli::parse();
        let default_limit: u8 = 5;
        let default_period = "7day".to_string();

        let final_limit = cli.limit.unwrap_or(default_limit);
        let final_period = cli.period.unwrap_or(default_period);

        Config {
            username: cli.username,
            limit: final_limit.to_string(),
            period: final_period,
        }
    }
}
