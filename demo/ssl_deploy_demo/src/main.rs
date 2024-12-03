mod aliyun;
mod config;
mod qiniu;

use aliyun::AliyunClient;
use qiniu::QiniuClient;

use anyhow::Result;
use clap::Parser;
use config::{Cli, Config};
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 构建日志
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // 获取配置
    let cli = Cli::parse();
    let config = Config::parse(cli.path).expect("配置文件不存在");
    info!("config: {:?}", config);

    // 阿里云：查询指定的证书申请订单的状态
    let aliyun_client = AliyunClient::new(&config.aliyun_access);
    // let res = aliyun_client.describe_certificate_state("12832873").await?;
    // info!("res: {:?}", res);

    // 七牛云：获取ssl证书详情
    let qiniu_client = QiniuClient::new(&config.qiniu_access);
    let cert = qiniu_client
        .get_cert_detail("66dea9f27a78350cca22ed4c")
        .await?;
    info!("sslcert: {:?}", cert);

    Ok(())
}
