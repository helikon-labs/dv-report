use crate::dv::delegation::Delegation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub twitter: Option<String>,
    pub delegation: Delegation,
}
