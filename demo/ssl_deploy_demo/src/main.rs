mod aliyun;
mod config;
mod qiniu;

use std::{thread, time::Duration};

use aliyun::{AliyunClient, CertificateStateResp, CreateCertificateReq, CreateCertificateResp};
use qiniu::{QiniuClient, QiniuHttpsConfRequest, QiniuUploadCertRequest};

use anyhow::{Error, Result};
use chrono::Local;
use clap::Parser;
use config::{Cli, Config};
use log::{error, info};

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

    let aliyun_client = AliyunClient::new(config.aliyun_access);

    // 创建证书
    let create_req = CreateCertificateReq {
        product_code: "digicert-free-1-free".to_string(),
        username: "蒙明宇".to_string(),
        phone: "18878613483".to_string(),
        email: "mmy6076@outlook.com".to_string(),
        domain: "oss.memoyu.com".to_string(),
        validate_type: "dns".to_string(),
    };
    let create_res = aliyun_client.create_certificate_request(create_req).await?;
    info!("create_res: {:?}", create_res);
    if create_res.order_id <= 0 {
        // error!("创建证书失败");
        return Err(Error::msg("创建证书失败"));
    }

    let mut cert_res: CertificateStateResp = CertificateStateResp {
        r#type: None,
        private_key: None,
        request_id: None,
        certificate: None,
    };
    while cert_res.private_key.is_none() {
        // 阿里云：查询指定的证书申请订单的状态
        let create_res = CreateCertificateResp {
            order_id: create_res.order_id,
            request_id: "test".to_string(),
        };
        cert_res = aliyun_client
            .describe_certificate_state(create_res.order_id)
            .await?;
        info!("cert_res: {:?}", cert_res);
        thread::sleep(Duration::from_secs(10));
    }

    let qiniu_client = QiniuClient::new(config.qiniu_access);
    // 七牛云：证书上传
    let today = Local::now().format("%Y-%m-%d").to_string();
    let cert_name = today + "-oss.memoyu.com";
    let cert_id: String;
    let qiniu_cert_upload_req = QiniuUploadCertRequest {
        name: cert_name.clone(),
        common_name: cert_name + "-cert",
        pri: cert_res.private_key.unwrap(),
        ca: cert_res.certificate.unwrap(),
    };
    let qiniu_cert_upload_res = qiniu_client.upload_cert(qiniu_cert_upload_req).await;
    cert_id = match qiniu_cert_upload_res {
        Ok(cert_id) => cert_id,
        Err(er) => {
            info!("error: {}", er);
            "".to_string()
        }
    };
    println!("七牛云上传证书成功 cert_id: {}", cert_id);

    // 七牛云：修改CDN SSL证书
    let https_conf_req = QiniuHttpsConfRequest {
        cert_id: cert_id,
        force_https: false,
        http2_enable: true,
    };
    qiniu_client
        .https_conf("oss.memoyu.com", https_conf_req)
        .await
        .unwrap();
    println!("七牛云修改CDN SSL证书成功");

    Ok(())
}
