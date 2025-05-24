use crate::dv::delegation::Delegation;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub twitter: Option<String>,
    pub delegation: Delegation,
}

#[derive(Debug, FromRow)]
pub struct DelegateRow {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub twitter: Option<String>,
}

impl DelegateRow {
    pub fn into_delegate(self, delegation: Delegation) -> Delegate {
        Delegate {
            id: self.id.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            twitter: self.twitter,
            delegation,
        }
    }
}
