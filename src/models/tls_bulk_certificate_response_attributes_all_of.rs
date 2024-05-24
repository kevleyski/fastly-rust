/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsBulkCertificateResponseAttributesAllOf {
    /// Time-stamp (GMT) when the certificate will expire. Must be in the future to be used to terminate TLS traffic.
    #[serde(rename = "not_after", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// Time-stamp (GMT) when the certificate will become valid. Must be in the past to be used to terminate TLS traffic.
    #[serde(rename = "not_before", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// A recommendation from Fastly indicating the key associated with this certificate is in need of rotation.
    #[serde(rename = "replace", skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

impl TlsBulkCertificateResponseAttributesAllOf {
    pub fn new() -> TlsBulkCertificateResponseAttributesAllOf {
        TlsBulkCertificateResponseAttributesAllOf {
            not_after: None,
            not_before: None,
            replace: None,
        }
    }
}


