// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.feed.getLikes` namespace.

pub trait GetLikes {
    fn get_likes(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cid: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub uri: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cid: Option<String>,
    pub cursor: Option<String>,
    pub likes: Vec<Like>,
    pub uri: String,
}

pub enum Error {
}

// app.bsky.feed.getLikes#like
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub actor: crate::app::bsky::actor::defs::ProfileView,
    pub created_at: String,
    pub indexed_at: String,
}
