use crate::dv::delegation::Delegation;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub url: Option<String>,
    pub twitter: Option<String>,
    pub delegations: Vec<Delegation>,
}

#[derive(Clone, Debug, FromRow)]
pub struct DelegateRow {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub url: Option<String>,
    pub twitter: Option<String>,
}

impl DelegateRow {
    pub fn into_delegate(self, delegations: Vec<Delegation>) -> Delegate {
        Delegate {
            id: self.id.clone(),
            name: self.name.clone(),
            short_name: self.short_name.clone(),
            url: self.url.clone(),
            twitter: self.twitter,
            delegations,
        }
    }
}
