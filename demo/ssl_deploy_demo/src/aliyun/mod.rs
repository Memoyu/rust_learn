use super::*;
use aliyun_openapi_core_rust_sdk::client::rpc::RPClient;
use serde::{Deserialize, Serialize};

pub struct AliyunClient<'a> {
    conf: &'a config::AccessConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CertificateState {
    r#type: String,
    private_key: String,
    request_id: String,
    certificate: String,
}

impl<'a> AliyunClient<'a> {
    pub fn new(conf: &'a config::AccessConfig) -> Self {
        AliyunClient { conf }
    }

    // 查询指定的证书申请订单的状态
    pub async fn describe_certificate_state(&self, order_id: &str) -> Result<CertificateState> {
        let aliyun_openapi_client = RPClient::new(
            self.conf.access_key.clone(),
            self.conf.access_secret.clone(),
            "https://cas.aliyuncs.com/",
        );

        let response = aliyun_openapi_client
            .version("2020-04-07")
            .get("DescribeCertificateState")
            .query([("OrderId", order_id)])
            .json::<CertificateState>()
            .await?;

        Ok(response)
    }
}
