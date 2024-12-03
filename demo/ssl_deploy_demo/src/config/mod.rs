use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(index = 1, required = true)]
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessConfig {
    pub access_key: String,
    pub access_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub aliyun_access: AccessConfig,
    pub qiniu_access: AccessConfig,
}

impl Config {
    // 解析配置文件
    pub fn parse(path: String) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
