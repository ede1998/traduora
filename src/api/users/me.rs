use crate::{api::Endpoint, auth::Authenticated, query::DefaultModel};
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
}

impl DefaultModel for Me {
    type Model = UserInfo;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub num_projects_created: u64,
}
