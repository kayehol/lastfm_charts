pub struct Config {
    pub username: String,
    pub option_flag: Option<String>,
    pub option_value: Option<String>,
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

        let option_flag = match args.next() {
            Some(arg) => Some(arg),
            None => None,
        };

        let option_value = match args.next() {
            Some(arg) => Some(arg),
            None => None,
        };

        Ok(Config {
            username,
            option_flag,
            option_value
        })
    }
}
