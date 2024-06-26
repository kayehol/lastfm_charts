use clap::Parser;

pub struct Config {
    pub username: String,
    pub limit: String,
}

#[derive(Parser)]
#[command(name = "LastFM Charts")]
#[command(version, about = "Get your weekly charts", long_about = None)]
struct Cli {
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    limit: Option<String>,
}

impl Config {
    pub fn build() -> Config {
        let cli = Cli::parse();
        let default_limit = "5".to_string();
        let final_limit = cli.limit.unwrap_or(default_limit);

        Config {
            username: cli.username,
            limit: final_limit,
        }
    }
}
