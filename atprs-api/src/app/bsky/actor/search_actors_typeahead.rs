// This file is generated by atprs-codegen. Do not edit.
//! Definitions for the `app.bsky.actor.searchActorsTypeahead` namespace.

/// Find actor suggestions for a search term.
pub trait SearchActorsTypeahead {
    fn search_actors_typeahead(&self, input: Parameters) -> Result<Output, Error>;
}

pub struct Parameters {
    pub limit: Option<i32>,
    pub term: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub actors: Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
}

pub enum Error {
}
