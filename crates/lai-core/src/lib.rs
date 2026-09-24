//! Native selection and permission-scoped projections. No I/O or provider code.
use std::collections::BTreeSet;
use lai_contracts::*;
use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct ClientPolicy {
    pub allow_passive_discovery:bool,
    /// None means all connection metadata; Some(empty) means none. Set only by the native host.
    pub connection_ids:Option<BTreeSet<String>>,
    pub reveal_identity:bool,
    pub probe_connection_ids:BTreeSet<String>,
}
impl ClientPolicy {
    pub fn denied()->Self{Self{connection_ids:Some(BTreeSet::new()),..Self::default()}}
    pub fn passive()->Self{Self{allow_passive_discovery:true,..Self::default()}}
    pub fn allows(&self,id:&str)->bool{self.connection_ids.as_ref().is_none_or(|ids|ids.contains(id))}
}
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct Requirements {
    #[serde(default)]pub required_capabilities:Vec<String>,
    #[serde(default)]pub allow_paid_fallback:bool,
    #[serde(default)]pub eligible_only:bool,
    #[serde(default)]pub billing_preference:Option<String>,
    #[serde(default)]pub query:Option<String>,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Candidate {pub selection:Selection,pub connection_id:String,pub eligible:bool,pub reasons:Vec<String>}
pub fn project_snapshot(_: &Snapshot,_:&ClientPolicy)->Snapshot{unimplemented!("test-first projection")}
pub fn resolve(_: &Snapshot,_:&Requirements)->Vec<Candidate>{unimplemented!("test-first native selection")}
pub fn validate_selection<'a>(_: &'a Snapshot,_:&Selection,_:bool)->Result<&'a Model>{unimplemented!("test-first runtime validation")}
