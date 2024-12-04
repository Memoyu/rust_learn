use super::*;
use anyhow::anyhow;
use qiniu_credential::{prelude::*, Credential, HeaderValue};
use serde::{Deserialize, Serialize};

pub struct QiniuClient {
    conf: config::AccessConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QiniuUploadCertRequest {
    pub name: String,
    pub common_name: String,
    pub pri: String,
    pub ca: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QiniuUploadCertResponse {
    pub code: i32,
    pub error: Option<String>,
    #[serde(rename = "certID")]
    pub cert_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct QiniuHttpsConfRequest {
    #[serde(rename = "certid")]
    pub cert_id: String,
    #[serde(rename = "forceHttps")]
    pub force_https: bool,
    #[serde(rename = "http2Enable")]
    pub http2_enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QiniuHttpsConfResponse {
    pub code: i32,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QiniuCertDetailResponse {
    pub code: i32,
    pub error: String,
    pub cert: QiniuCertDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QiniuCertDetail {
    name: String,
    common_name: String,
    dnsnames: Vec<String>,
    not_before: i32,
    not_after: i32,
    pri: String,
    ca: String,
    create_time: i32,
}

impl QiniuClient {
    pub fn new(conf: config::AccessConfig) -> Self {
        QiniuClient { conf }
    }

    // 获取管理凭证
    fn get_authorization(
        &self,
        url: &str,
        content_type: Option<&HeaderValue>,
        body: Option<&[u8]>,
    ) -> Result<String> {
        let credential = Credential::new(&self.conf.access_key, &self.conf.access_secret);

        let req = match body {
            Some(b) => b,
            _ => "".as_bytes(),
        };

        let authorization = credential
            .get(Default::default())?
            .authorization_v1_for_request(&url.parse()?, content_type, req);

        Ok(authorization)
    }

    // 获取证书详情
    pub async fn get_cert_detail(&self, cred_id: &str) -> Result<QiniuCertDetail> {
        let url = format!("http://api.qiniu.com/sslcert/{}", cred_id);

        let authorization = self.get_authorization(&url, None, None)?;

        let client = reqwest::Client::new();
        let detail = client
            .get(url)
            .header("Authorization", authorization)
            .send()
            .await?
            .json::<QiniuCertDetailResponse>()
            .await?;

        Ok(detail.cert)
    }

    // 上传证书
    pub async fn upload_cert(&self, req: QiniuUploadCertRequest) -> Result<String, anyhow::Error> {
        let url = "http://api.qiniu.com/sslcert";

        let json = serde_json::to_string(&req).unwrap();
        // println!("json: {}", json);
        let authorization = self.get_authorization(
            &url,
            Some(&HeaderValue::from_str("application/json").unwrap()),
            Some(json.as_bytes()),
        )?;

        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .json(&req)
            .header("Authorization", authorization)
            .send()
            .await?
            .json::<QiniuUploadCertResponse>()
            .await?;
        if res.code != 200 {
            return Err(anyhow!(
                "上传证书错误：{}",
                res.error.unwrap_or("请求异常".to_string())
            ));
        }

        Ok(res.cert_id)
    }

    // 修改CDN证书配置
    pub async fn https_conf(
        &self,
        domain: &str,
        req: QiniuHttpsConfRequest,
    ) -> Result<(), anyhow::Error> {
        let url = format!("http://api.qiniu.com/domain/{}/httpsconf", domain);

        let json = serde_json::to_string(&req).unwrap();
        println!("json: {}", json);
        let authorization = self.get_authorization(
            &url,
            Some(&HeaderValue::from_str("application/json").unwrap()),
            Some(json.as_bytes()),
        )?;

        let client = reqwest::Client::new();
        let res = client
            .put(url)
            .json(&req)
            .header("Authorization", authorization)
            .send()
            .await?
            .json::<QiniuHttpsConfResponse>()
            .await?;
        println!("https_conf: {:?}", res);
        if res.code != 200 {
            return Err(anyhow!(
                "修改CDN证书配置错误：{}",
                res.error.unwrap_or("请求异常".to_string())
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_client() -> QiniuClient {
        let config = Config::parse("./config.toml".to_string()).unwrap();
        QiniuClient::new(config.qiniu_access)
    }

    #[tokio::test]
    async fn upload_cert_test() {
        let qiniu_client = get_client();

        let req = QiniuUploadCertRequest {
            name: "oss.memoyu.com".to_string(),
            common_name: "api_upload_cert_common".to_string(),
            pri: "-----BEGIN RSA PRIVATE KEY-----\nMIIEogIBAAKCAQEArgRlnlBhLHY+y2q7MU74jgafM40ob6BpFTAEAOsfsrw3+PH4\n8TgUGkB8G+o7/ySDWzivKZ+/NrNvW8ciMxoSu48+DPa8qXVSH3QY4cjdz/Zhxg1Q\ncB05IbBfRIFlfwnuekMI8i1+vhGD9ZudUX3VdIKEc8MllAIRtSjNYKocc80ZmIcx\n/M43nXTuIFvFx6UTe6GRKvcj4SHc+bdyrsW7vqaP9O9XM3woVSxD2WM4fsXcdrfi\n3N26lHnMG0tfU4mOlfGQZNQ+t/l8t+rOUDmGvwDZu6DI0lEY8BtWxjMaSiKg5BMu\n0IXj4k/qq30zApHXSvU5KLouUKbMsFyl+g7aywIDAQABAoIBABEIrCZ5lBwv554D\nhhcMYGc3/bh5nlYtm8EFBgyi92OIahWLy28CqsZrvoUAV5F2R7ITvo2yrfB9qFkR\ns+bQVroWX6ARkk5821Dp4o+jbmxaH/Ws6E1OhwD7DJrzlvWptAwyvfrwJM0X1ioi\nFUhcqsWGCyTGcEoTLnGPG2oqwgXnCeRnQbhzK4h52lm44iXAbOpMtMD5GNtxNOvc\nYCBsvspVYFnRu834IAuuJOXXOVTL7VWn5uVYAbH9GeRfIp+wdvVPf7bXQyD4kf1t\ncMKVphRAUBgBklHgENp0nWkd/6J1FTRoaDeKGm5vRqG040jsvGOqmfeFEzu2RrFl\nuoYsnRkCgYEA2+SklNl+Y0fhfr8eb/IdomTfBUoRYMNaVO5qbLpT4b6XYHd5h048\nMhS/o7UTq/qCY1qDU6OXpIOnZLmakoJ70BuBtr7FUF7oURZDoxPwttNPBL7pYW02\npD7RtrfPUroIHxzhLgyBi5ME7g5to+P5umQ7QfIWVrCVT3V0Nwsg/1kCgYEAypdT\nsbiMmXRjMI8dZyCx1108GKGRH3S3rl0syyEBFxXtcCfaYCAnL8vhLEODDaSmWn/Y\nrNZJHF1DIM2j7/lBP9qfv7/x0IkxFUIFvL408+SMmzzpOC6p4VBzoYscdV9uIQOz\nZ5jz8DcD9zvPell8tI1uhpokfL/HiqNX9XcN6sMCgYBVLuHC9+5KL5JfM1JrM3Pf\ncQwkVGke/w6DUEjSDexoyxiBOkdN7zTKgdcrZ/5S5RyVuH6fbARWqtmx32TN1NZ5\n6+34CTcOx3Vf1CwnhJX/fSRjRkHKiDBeuCctjU83UPHMe7ePyzB0DWQfhF9c2ffo\nCOQby9hzhLySp4F5wUnrIQKBgFJ+OCg/25CI4fLd870QhRoK7vOgpnVGk5y+eATm\nlYkO9rYapENMFGjT/gaGHxEpIEvtjw8h2ndgSntLARwwRK95mF7+C7s8ge4ECk4G\nrY06XT7BGL410hrbJZp8CMSKbU+oMB9ZGsuPu1+qzRsm4S6sAHtyvLTHars7NA/R\nwKr9AoGANJG4cFa+dnmLArh1YQSEwHddozcL5Y9YKKOMCmFJbmXcmvoqm8K7RZgE\nrGztCWLcOCo8EvyOJgL43VwJdz//2xlgQwYgTlupYeb/gTg0fkZ/Rk75XOI5k3TG\n0pPJ857zV1K4rwaDE4a2hs/jrIfuONfk0tu8e/zzz3ykW/L5vy8=\n-----END RSA PRIVATE KEY-----".to_string(),
            ca: "-----BEGIN CERTIFICATE-----\nMIIF9TCCBN2gAwIBAgIQB0E6K1edarB3xAoiNypouDANBgkqhkiG9w0BAQsFADBu\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMS0wKwYDVQQDEyRFbmNyeXB0aW9uIEV2ZXJ5d2hlcmUg\nRFYgVExTIENBIC0gRzIwHhcNMjQxMTMwMDAwMDAwWhcNMjUwMjI4MjM1OTU5WjAZ\nMRcwFQYDVQQDEw5vc3MubWVtb3l1LmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEP\nADCCAQoCggEBAK4EZZ5QYSx2PstquzFO+I4GnzONKG+gaRUwBADrH7K8N/jx+PE4\nFBpAfBvqO/8kg1s4rymfvzazb1vHIjMaEruPPgz2vKl1Uh90GOHI3c/2YcYNUHAd\nOSGwX0SBZX8J7npDCPItfr4Rg/WbnVF91XSChHPDJZQCEbUozWCqHHPNGZiHMfzO\nN5107iBbxcelE3uhkSr3I+Eh3Pm3cq7Fu76mj/TvVzN8KFUsQ9ljOH7F3Ha34tzd\nupR5zBtLX1OJjpXxkGTUPrf5fLfqzlA5hr8A2bugyNJRGPAbVsYzGkoioOQTLtCF\n4+JP6qt9MwKR10r1OSi6LlCmzLBcpfoO2ssCAwEAAaOCAuIwggLeMB8GA1UdIwQY\nMBaAFHjfkZBf7t6s9sV169VMVVPvJEq2MB0GA1UdDgQWBBRc1d0kKBTciKIb7hct\nLqCvYTsIbTAZBgNVHREEEjAQgg5vc3MubWVtb3l1LmNvbTA+BgNVHSAENzA1MDMG\nBmeBDAECATApMCcGCCsGAQUFBwIBFhtodHRwOi8vd3d3LmRpZ2ljZXJ0LmNvbS9D\nUFMwDgYDVR0PAQH/BAQDAgWgMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcD\nAjCBgAYIKwYBBQUHAQEEdDByMCQGCCsGAQUFBzABhhhodHRwOi8vb2NzcC5kaWdp\nY2VydC5jb20wSgYIKwYBBQUHMAKGPmh0dHA6Ly9jYWNlcnRzLmRpZ2ljZXJ0LmNv\nbS9FbmNyeXB0aW9uRXZlcnl3aGVyZURWVExTQ0EtRzIuY3J0MAwGA1UdEwEB/wQC\nMAAwggF/BgorBgEEAdZ5AgQCBIIBbwSCAWsBaQB3AE51oydcmhDDOFts1N8/Uusd\n8OCOG41pwLH6ZLFimjnfAAABk3uRqZsAAAQDAEgwRgIhAJufdb5O37Iftat8RTTU\nIQje72dqYI0ck2VOk2xE9Sc5AiEAqaZ221fdzAwQahYz3ncF66ehYA0Bk2Huwnvn\nWi7nMuwAdwBzICIPCBaK+fPEposKsmqaSgDu9XeFighNBQDUpUJEWQAAAZN7kali\nAAAEAwBIMEYCIQCBLSoTBVU4bBO4C91uD2uB0SSFOS0xgfNQL4UXVGscIAIhAO1P\nG0ZgRTHmzMPNZ46TraUVxF9pwH2juMGUR5waha3VAHUA5tIxY0B3jMEQQQbXcbnO\nwdJA9paEhvu6hzId/R43jlAAAAGTe5GpeAAABAMARjBEAiBWgCpZRAP/d9a8UzJj\nGkWtl1BumZBFxV+peoEZ2qR2lgIgJeEH6gef2R/Ts1G3LRxQH1+EwUR6uDsxUkne\nhyDy43kwDQYJKoZIhvcNAQELBQADggEBAAd7kDWu42O22+veMVZVOCeugYlGkj4Q\nogLeYTd7Tt8gdPB6cpDcQ0N3L1a7CU8ipTjVsyR10L+TCK2+EfGMAgt6Tff0HMxz\nAToG+SfjGnhQiTFIKLVb19oEx4NRNb3SPtie8B5L8dy9t7lHfxSvJY2HUOjfU5Bn\nD2Hquo29mPjvM+pcQwa/NO+V3HuwYg78LpYTp8vNCknLcz36xpMQfOre3tNLA+1l\nqB4RzUIrmNawaIulzsjuZVevn9jsSA2ec+NqXCyWfMGdujSN1PZYpks6nKUQ6bpi\nfW7jxH1HFBwTxDv3Vfxlblz031h4KdlfeFmLuN5N6xDvgmqYf2Y7YyI=\n-----END CERTIFICATE-----\n-----BEGIN CERTIFICATE-----\nMIIEqjCCA5KgAwIBAgIQDeD/te5iy2EQn2CMnO1e0zANBgkqhkiG9w0BAQsFADBh\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSAwHgYDVQQDExdEaWdpQ2VydCBHbG9iYWwgUm9vdCBH\nMjAeFw0xNzExMjcxMjQ2NDBaFw0yNzExMjcxMjQ2NDBaMG4xCzAJBgNVBAYTAlVT\nMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdpY2VydC5j\nb20xLTArBgNVBAMTJEVuY3J5cHRpb24gRXZlcnl3aGVyZSBEViBUTFMgQ0EgLSBH\nMjCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAO8Uf46i/nr7pkgTDqnE\neSIfCFqvPnUq3aF1tMJ5hh9MnO6Lmt5UdHfBGwC9Si+XjK12cjZgxObsL6Rg1njv\nNhAMJ4JunN0JGGRJGSevbJsA3sc68nbPQzuKp5Jc8vpryp2mts38pSCXorPR+sch\nQisKA7OSQ1MjcFN0d7tbrceWFNbzgL2csJVQeogOBGSe/KZEIZw6gXLKeFe7mupn\nNYJROi2iC11+HuF79iAttMc32Cv6UOxixY/3ZV+LzpLnklFq98XORgwkIJL1HuvP\nha8yvb+W6JislZJL+HLFtidoxmI7Qm3ZyIV66W533DsGFimFJkz3y0GeHWuSVMbI\nlfsCAwEAAaOCAU8wggFLMB0GA1UdDgQWBBR435GQX+7erPbFdevVTFVT7yRKtjAf\nBgNVHSMEGDAWgBROIlQgGJXm427mD/r6uRLtBhePOTAOBgNVHQ8BAf8EBAMCAYYw\nHQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMBIGA1UdEwEB/wQIMAYBAf8C\nAQAwNAYIKwYBBQUHAQEEKDAmMCQGCCsGAQUFBzABhhhodHRwOi8vb2NzcC5kaWdp\nY2VydC5jb20wQgYDVR0fBDswOTA3oDWgM4YxaHR0cDovL2NybDMuZGlnaWNlcnQu\nY29tL0RpZ2lDZXJ0R2xvYmFsUm9vdEcyLmNybDBMBgNVHSAERTBDMDcGCWCGSAGG\n/WwBAjAqMCgGCCsGAQUFBwIBFhxodHRwczovL3d3dy5kaWdpY2VydC5jb20vQ1BT\nMAgGBmeBDAECATANBgkqhkiG9w0BAQsFAAOCAQEAoBs1eCLKakLtVRPFRjBIJ9LJ\nL0s8ZWum8U8/1TMVkQMBn+CPb5xnCD0GSA6L/V0ZFrMNqBirrr5B241OesECvxIi\n98bZ90h9+q/X5eMyOD35f8YTaEMpdnQCnawIwiHx06/0BfiTj+b/XQih+mqt3ZXe\nxNCJqKexdiB2IWGSKcgahPacWkk/BAQFisKIFYEqHzV974S3FAz/8LIfD58xnsEN\nGfzyIDkH3JrwYZ8caPTf6ZX9M1GrISN8HnWTtdNCH2xEajRa/h9ZBXjUyFKQrGk2\nn2hcLrfZSbynEC/pSw/ET7H5nWwckjmAJ1l9fcnbqkU/pf6uMQmnfl0JQjJNSg==\n-----END CERTIFICATE-----".to_string(),
        };

        let cert_id = qiniu_client.upload_cert(req).await.unwrap();
        println!("cert_id: {:?}", cert_id);
    }

    #[tokio::test]
    async fn https_conf_test() {
        let qiniu_client = get_client();

        let req = QiniuHttpsConfRequest {
            cert_id: "674ffff557a5f2a2f6718f6f".to_string(),
            force_https: false,
            http2_enable: true,
        };

        qiniu_client
            .https_conf("oss.memoyu.com", req)
            .await
            .unwrap();
    }
}
