mod aliyun;
mod config;
mod qiniu;

use aliyun::AliyunClient;
use qiniu::{QiniuClient, QiniuUploadCertRequest};

use anyhow::Result;
use chrono::Local;
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
    let aliyun_client = AliyunClient::new(config.aliyun_access);
    let res = aliyun_client.describe_certificate_state("12832873").await?;
    info!("res: {:?}", res);

    // 七牛云：获取ssl证书详情
    let qiniu_client = QiniuClient::new(config.qiniu_access);
    let today = Local::now().format("%Y-%m-%d").to_string();
    let cert_name = today + "-oss.memoyu.com";

    let req = QiniuUploadCertRequest {
        name: cert_name.clone(),
        common_name: cert_name + "-cert",
        pri: "-----BEGIN RSA PRIVATE KEY-----\nMIIEogIBAAKCAQEArgRlnlBhLHY+y2q7MU74jgafM40ob6BpFTAEAOsfsrw3+PH4\n8TgUGkB8G+o7/ySDWzivKZ+/NrNvW8ciMxoSu48+DPa8qXVSH3QY4cjdz/Zhxg1Q\ncB05IbBfRIFlfwnuekMI8i1+vhGD9ZudUX3VdIKEc8MllAIRtSjNYKocc80ZmIcx\n/M43nXTuIFvFx6UTe6GRKvcj4SHc+bdyrsW7vqaP9O9XM3woVSxD2WM4fsXcdrfi\n3N26lHnMG0tfU4mOlfGQZNQ+t/l8t+rOUDmGvwDZu6DI0lEY8BtWxjMaSiKg5BMu\n0IXj4k/qq30zApHXSvU5KLouUKbMsFyl+g7aywIDAQABAoIBABEIrCZ5lBwv554D\nhhcMYGc3/bh5nlYtm8EFBgyi92OIahWLy28CqsZrvoUAV5F2R7ITvo2yrfB9qFkR\ns+bQVroWX6ARkk5821Dp4o+jbmxaH/Ws6E1OhwD7DJrzlvWptAwyvfrwJM0X1ioi\nFUhcqsWGCyTGcEoTLnGPG2oqwgXnCeRnQbhzK4h52lm44iXAbOpMtMD5GNtxNOvc\nYCBsvspVYFnRu834IAuuJOXXOVTL7VWn5uVYAbH9GeRfIp+wdvVPf7bXQyD4kf1t\ncMKVphRAUBgBklHgENp0nWkd/6J1FTRoaDeKGm5vRqG040jsvGOqmfeFEzu2RrFl\nuoYsnRkCgYEA2+SklNl+Y0fhfr8eb/IdomTfBUoRYMNaVO5qbLpT4b6XYHd5h048\nMhS/o7UTq/qCY1qDU6OXpIOnZLmakoJ70BuBtr7FUF7oURZDoxPwttNPBL7pYW02\npD7RtrfPUroIHxzhLgyBi5ME7g5to+P5umQ7QfIWVrCVT3V0Nwsg/1kCgYEAypdT\nsbiMmXRjMI8dZyCx1108GKGRH3S3rl0syyEBFxXtcCfaYCAnL8vhLEODDaSmWn/Y\nrNZJHF1DIM2j7/lBP9qfv7/x0IkxFUIFvL408+SMmzzpOC6p4VBzoYscdV9uIQOz\nZ5jz8DcD9zvPell8tI1uhpokfL/HiqNX9XcN6sMCgYBVLuHC9+5KL5JfM1JrM3Pf\ncQwkVGke/w6DUEjSDexoyxiBOkdN7zTKgdcrZ/5S5RyVuH6fbARWqtmx32TN1NZ5\n6+34CTcOx3Vf1CwnhJX/fSRjRkHKiDBeuCctjU83UPHMe7ePyzB0DWQfhF9c2ffo\nCOQby9hzhLySp4F5wUnrIQKBgFJ+OCg/25CI4fLd870QhRoK7vOgpnVGk5y+eATm\nlYkO9rYapENMFGjT/gaGHxEpIEvtjw8h2ndgSntLARwwRK95mF7+C7s8ge4ECk4G\nrY06XT7BGL410hrbJZp8CMSKbU+oMB9ZGsuPu1+qzRsm4S6sAHtyvLTHars7NA/R\nwKr9AoGANJG4cFa+dnmLArh1YQSEwHddozcL5Y9YKKOMCmFJbmXcmvoqm8K7RZgE\nrGztCWLcOCo8EvyOJgL43VwJdz//2xlgQwYgTlupYeb/gTg0fkZ/Rk75XOI5k3TG\n0pPJ857zV1K4rwaDE4a2hs/jrIfuONfk0tu8e/zzz3ykW/L5vy8=\n-----END RSA PRIVATE KEY-----".to_string(),
        ca: "-----BEGIN CERTIFICATE-----\nMIIF9TCCBN2gAwIBAgIQB0E6K1edarB3xAoiNypouDANBgkqhkiG9w0BAQsFADBu\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMS0wKwYDVQQDEyRFbmNyeXB0aW9uIEV2ZXJ5d2hlcmUg\nRFYgVExTIENBIC0gRzIwHhcNMjQxMTMwMDAwMDAwWhcNMjUwMjI4MjM1OTU5WjAZ\nMRcwFQYDVQQDEw5vc3MubWVtb3l1LmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEP\nADCCAQoCggEBAK4EZZ5QYSx2PstquzFO+I4GnzONKG+gaRUwBADrH7K8N/jx+PE4\nFBpAfBvqO/8kg1s4rymfvzazb1vHIjMaEruPPgz2vKl1Uh90GOHI3c/2YcYNUHAd\nOSGwX0SBZX8J7npDCPItfr4Rg/WbnVF91XSChHPDJZQCEbUozWCqHHPNGZiHMfzO\nN5107iBbxcelE3uhkSr3I+Eh3Pm3cq7Fu76mj/TvVzN8KFUsQ9ljOH7F3Ha34tzd\nupR5zBtLX1OJjpXxkGTUPrf5fLfqzlA5hr8A2bugyNJRGPAbVsYzGkoioOQTLtCF\n4+JP6qt9MwKR10r1OSi6LlCmzLBcpfoO2ssCAwEAAaOCAuIwggLeMB8GA1UdIwQY\nMBaAFHjfkZBf7t6s9sV169VMVVPvJEq2MB0GA1UdDgQWBBRc1d0kKBTciKIb7hct\nLqCvYTsIbTAZBgNVHREEEjAQgg5vc3MubWVtb3l1LmNvbTA+BgNVHSAENzA1MDMG\nBmeBDAECATApMCcGCCsGAQUFBwIBFhtodHRwOi8vd3d3LmRpZ2ljZXJ0LmNvbS9D\nUFMwDgYDVR0PAQH/BAQDAgWgMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcD\nAjCBgAYIKwYBBQUHAQEEdDByMCQGCCsGAQUFBzABhhhodHRwOi8vb2NzcC5kaWdp\nY2VydC5jb20wSgYIKwYBBQUHMAKGPmh0dHA6Ly9jYWNlcnRzLmRpZ2ljZXJ0LmNv\nbS9FbmNyeXB0aW9uRXZlcnl3aGVyZURWVExTQ0EtRzIuY3J0MAwGA1UdEwEB/wQC\nMAAwggF/BgorBgEEAdZ5AgQCBIIBbwSCAWsBaQB3AE51oydcmhDDOFts1N8/Uusd\n8OCOG41pwLH6ZLFimjnfAAABk3uRqZsAAAQDAEgwRgIhAJufdb5O37Iftat8RTTU\nIQje72dqYI0ck2VOk2xE9Sc5AiEAqaZ221fdzAwQahYz3ncF66ehYA0Bk2Huwnvn\nWi7nMuwAdwBzICIPCBaK+fPEposKsmqaSgDu9XeFighNBQDUpUJEWQAAAZN7kali\nAAAEAwBIMEYCIQCBLSoTBVU4bBO4C91uD2uB0SSFOS0xgfNQL4UXVGscIAIhAO1P\nG0ZgRTHmzMPNZ46TraUVxF9pwH2juMGUR5waha3VAHUA5tIxY0B3jMEQQQbXcbnO\nwdJA9paEhvu6hzId/R43jlAAAAGTe5GpeAAABAMARjBEAiBWgCpZRAP/d9a8UzJj\nGkWtl1BumZBFxV+peoEZ2qR2lgIgJeEH6gef2R/Ts1G3LRxQH1+EwUR6uDsxUkne\nhyDy43kwDQYJKoZIhvcNAQELBQADggEBAAd7kDWu42O22+veMVZVOCeugYlGkj4Q\nogLeYTd7Tt8gdPB6cpDcQ0N3L1a7CU8ipTjVsyR10L+TCK2+EfGMAgt6Tff0HMxz\nAToG+SfjGnhQiTFIKLVb19oEx4NRNb3SPtie8B5L8dy9t7lHfxSvJY2HUOjfU5Bn\nD2Hquo29mPjvM+pcQwa/NO+V3HuwYg78LpYTp8vNCknLcz36xpMQfOre3tNLA+1l\nqB4RzUIrmNawaIulzsjuZVevn9jsSA2ec+NqXCyWfMGdujSN1PZYpks6nKUQ6bpi\nfW7jxH1HFBwTxDv3Vfxlblz031h4KdlfeFmLuN5N6xDvgmqYf2Y7YyI=\n-----END CERTIFICATE-----\n-----BEGIN CERTIFICATE-----\nMIIEqjCCA5KgAwIBAgIQDeD/te5iy2EQn2CMnO1e0zANBgkqhkiG9w0BAQsFADBh\nMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3\nd3cuZGlnaWNlcnQuY29tMSAwHgYDVQQDExdEaWdpQ2VydCBHbG9iYWwgUm9vdCBH\nMjAeFw0xNzExMjcxMjQ2NDBaFw0yNzExMjcxMjQ2NDBaMG4xCzAJBgNVBAYTAlVT\nMRUwEwYDVQQKEwxEaWdpQ2VydCBJbmMxGTAXBgNVBAsTEHd3dy5kaWdpY2VydC5j\nb20xLTArBgNVBAMTJEVuY3J5cHRpb24gRXZlcnl3aGVyZSBEViBUTFMgQ0EgLSBH\nMjCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAO8Uf46i/nr7pkgTDqnE\neSIfCFqvPnUq3aF1tMJ5hh9MnO6Lmt5UdHfBGwC9Si+XjK12cjZgxObsL6Rg1njv\nNhAMJ4JunN0JGGRJGSevbJsA3sc68nbPQzuKp5Jc8vpryp2mts38pSCXorPR+sch\nQisKA7OSQ1MjcFN0d7tbrceWFNbzgL2csJVQeogOBGSe/KZEIZw6gXLKeFe7mupn\nNYJROi2iC11+HuF79iAttMc32Cv6UOxixY/3ZV+LzpLnklFq98XORgwkIJL1HuvP\nha8yvb+W6JislZJL+HLFtidoxmI7Qm3ZyIV66W533DsGFimFJkz3y0GeHWuSVMbI\nlfsCAwEAAaOCAU8wggFLMB0GA1UdDgQWBBR435GQX+7erPbFdevVTFVT7yRKtjAf\nBgNVHSMEGDAWgBROIlQgGJXm427mD/r6uRLtBhePOTAOBgNVHQ8BAf8EBAMCAYYw\nHQYDVR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMBIGA1UdEwEB/wQIMAYBAf8C\nAQAwNAYIKwYBBQUHAQEEKDAmMCQGCCsGAQUFBzABhhhodHRwOi8vb2NzcC5kaWdp\nY2VydC5jb20wQgYDVR0fBDswOTA3oDWgM4YxaHR0cDovL2NybDMuZGlnaWNlcnQu\nY29tL0RpZ2lDZXJ0R2xvYmFsUm9vdEcyLmNybDBMBgNVHSAERTBDMDcGCWCGSAGG\n/WwBAjAqMCgGCCsGAQUFBwIBFhxodHRwczovL3d3dy5kaWdpY2VydC5jb20vQ1BT\nMAgGBmeBDAECATANBgkqhkiG9w0BAQsFAAOCAQEAoBs1eCLKakLtVRPFRjBIJ9LJ\nL0s8ZWum8U8/1TMVkQMBn+CPb5xnCD0GSA6L/V0ZFrMNqBirrr5B241OesECvxIi\n98bZ90h9+q/X5eMyOD35f8YTaEMpdnQCnawIwiHx06/0BfiTj+b/XQih+mqt3ZXe\nxNCJqKexdiB2IWGSKcgahPacWkk/BAQFisKIFYEqHzV974S3FAz/8LIfD58xnsEN\nGfzyIDkH3JrwYZ8caPTf6ZX9M1GrISN8HnWTtdNCH2xEajRa/h9ZBXjUyFKQrGk2\nn2hcLrfZSbynEC/pSw/ET7H5nWwckjmAJ1l9fcnbqkU/pf6uMQmnfl0JQjJNSg==\n-----END CERTIFICATE-----".to_string(),
    };

    let cert_upload = qiniu_client.upload_cert(req).await;
    let cert_id = match cert_upload {
        Ok(cert_id) => cert_id,
        Err(er) => {
            info!("error: {}", er);
            "".to_string()
        }
    };

    Ok(())
}
