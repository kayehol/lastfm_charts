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
    #[arg(short, long)]
    limit: Option<String>,
    #[arg(short, long)]
    period: Option<String>,
}

impl Config {
    pub fn build() -> Config {
        let cli = Cli::parse();
        let default_limit = "5".to_string();
        let default_period = "7day".to_string();

        let final_limit = cli.limit.unwrap_or(default_limit);
        let final_period = cli.period.unwrap_or(default_period);

        Config {
            username: cli.username,
            limit: final_limit,
            period: final_period,
        }
    }
}
