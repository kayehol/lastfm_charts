pub struct Config {
    pub username: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let username = args[1].clone();

        Ok(Config {
            username
        })
    }
}
