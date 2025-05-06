use std::collections::HashMap;

use super::*;
use aliyun_openapi_core_rust_sdk::client::rpc::RPClient;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};

pub struct AliyunClient {
    conf: config::AccessConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CertificateStateResp {
    pub r#type: Option<String>,
    pub private_key: Option<String>,
    pub request_id: Option<String>,
    pub certificate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCertificateReq {
    pub product_code: String,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub domain: String,
    pub validate_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCertificateResp {
    pub order_id: i64,
    pub request_id: String,
}

impl AliyunClient {
    pub fn new(conf: config::AccessConfig) -> Self {
        AliyunClient { conf }
    }

    // 完成DV证书购买申请和签发流程
    pub async fn create_certificate_request(
        &self,
        req: CreateCertificateReq,
    ) -> Result<CreateCertificateResp> {
        let aliyun_openapi_client = self.get_client();

        let mut params = HashMap::new();
        params.insert("ProductCode", req.product_code.as_str());
        params.insert("Username", req.username.as_str());
        params.insert("Phone", req.phone.as_str());
        params.insert("Email", req.email.as_str());
        params.insert("Domain", req.domain.as_str());
        params.insert("ValidateType", req.validate_type.as_str());

        let response = aliyun_openapi_client
            .version("2020-04-07")
            .post("CreateCertificateRequest")
            .query(params.into_iter())
            .json::<CreateCertificateResp>()
            .await?;

        Ok(response)
    }

    // 查询指定的证书申请订单的状态
    pub async fn describe_certificate_state(&self, order_id: i64) -> Result<CertificateStateResp> {
        let aliyun_openapi_client = self.get_client();

        let response = aliyun_openapi_client
            .version("2020-04-07")
            .get("DescribeCertificateState")
            .query([("OrderId", order_id.to_string().as_str())])
            .json::<CertificateStateResp>()
            .await?;

        Ok(response)
    }

    fn get_client(&self) -> RPClient {
        RPClient::new(
            self.conf.access_key.clone(),
            self.conf.access_secret.clone(),
            "https://cas.aliyuncs.com/",
        )
    }
}
