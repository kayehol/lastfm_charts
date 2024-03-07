pub struct Config {
    pub username: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let username = match args.next() {
            Some(arg) => arg,
            None => return Err("Please pass username as an argument"),
        };

        Ok(Config {
            username
        })
    }
}
