use crate::{
    api::Endpoint,
    auth::Authenticated,
    fetch::{AsyncFetcher, Fetcher},
};
use serde::Deserialize;

pub struct Me;

impl Endpoint for Me {
    type AccessControl = Authenticated;

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "users/me".into()
    }

    type Body = UserInfo;
}

impl Fetcher for Me {}
impl AsyncFetcher for Me {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub num_projects_created: u64,
}
