// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.graph.getMutes` namespace.

/// Who does the viewer mute?
pub trait GetMutes {
    fn get_mutes(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub cursor: Option<String>,
    pub mutes: Vec<crate::app::bsky::actor::defs::ProfileView>,
}

pub enum Error {
}
