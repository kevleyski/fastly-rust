/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// WsMessageFormat : Payload format for delivering to subscribers of WebSocket messages. One of `content` or `content-bin` must be specified.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WsMessageFormat {
    /// The content of a WebSocket `TEXT` message.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The base64-encoded content of a WebSocket `BINARY` message.
    #[serde(rename = "content-bin", skip_serializing_if = "Option::is_none")]
    pub content_bin: Option<String>,
}

impl WsMessageFormat {
    /// Payload format for delivering to subscribers of WebSocket messages. One of `content` or `content-bin` must be specified.
    pub fn new() -> WsMessageFormat {
        WsMessageFormat {
            content: None,
            content_bin: None,
        }
    }
}


