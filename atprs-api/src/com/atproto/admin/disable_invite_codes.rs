// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `com.atproto.admin.disableInviteCodes` namespace.

/// Disable some set of codes and/or all codes associated with a set of users
#[async_trait::async_trait]
pub trait DisableInviteCodes: crate::xrpc::XrpcClient {
    async fn disable_invite_codes(&self, input: Input) -> Result<(), Box<dyn std::error::Error>> {
        crate::xrpc::XrpcClient::send(
            self,
            http::Method::POST,
            "com.atproto.admin.disableInviteCodes",
            Some(input),
        )
        .await
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub accounts: Option<Vec<String>>,
    pub codes: Option<Vec<String>>,
}

pub enum Error {
}
