use std::fs;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub aliyun: Aliyun,
    pub telegram: Telegram
}

#[derive(Debug, Deserialize)]
pub struct Aliyun {
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct Telegram {
   pub token: String,
   pub chat_id: String
}

impl Config {
    pub fn new(file: &str) -> anyhow::Result<Config> {
        let content = fs::read_to_string(file)?;
        let config: Config = toml::from_str(content.as_str())?;
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn test_config(){
        let c = Config::new("demo.toml");   
        assert!(c.is_ok());
        assert_eq!(c.unwrap().aliyun.token, "x");
    }
}