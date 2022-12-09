/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateBillingAddressRequest {
    /// When set to true, the address will be saved without verification
    #[serde(rename = "skip_verification", skip_serializing_if = "Option::is_none")]
    pub skip_verification: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::UpdateBillingAddressRequestData>>,
}

impl UpdateBillingAddressRequest {
    pub fn new() -> UpdateBillingAddressRequest {
        UpdateBillingAddressRequest {
            skip_verification: None,
            data: None,
        }
    }
}


