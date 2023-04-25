// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.notification.listNotifications` namespace.

pub trait ListNotifications {
    fn list_notifications(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub seen_at: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cursor: Option<String>,
    pub notifications: Vec<Notification>,
}

pub enum Error {
}

// app.bsky.notification.listNotifications#notification
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub author: crate::app::bsky::actor::defs::ProfileView,
    pub cid: String,
    pub indexed_at: String,
    pub is_read: bool,
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    /// Expected values are 'like', 'repost', 'follow', 'mention', 'reply', and 'quote'.
    pub reason: String,
    pub reason_subject: Option<String>,
    // pub record: ...,
    pub uri: String,
}
