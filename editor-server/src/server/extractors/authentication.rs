use crate::global::APP_CLIENTS;
use crate::types::global::UserContext;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Authencate user by token stored in cookie
/// Then pass a user context to the handler
#[derive(Debug)]
pub struct Authentication(pub UserContext);

#[derive(Debug, Deserialize, Serialize)]
struct Token(String);

#[async_trait]
impl FromRequestParts<()> for Authentication {
    type Rejection = &'static str;

    async fn from_request_parts(_parts: &mut Parts, _state: &()) -> Result<Self, Self::Rejection> {
        // Load cookie jar for fetching token
        let cookie_jar = CookieJar::from_request_parts(_parts, &()).await;
        let cookie_jar = match cookie_jar {
            Ok(cookie_jar) => cookie_jar,
            Err(_) => return Err("No cookie jar"),
        };

        #[allow(unused)]
        let token = match cookie_jar.get("token") {
            Some(token) => token.value().to_string(),
            None => return Err("No token"),
        };

        let app_state = APP_CLIENTS.get().unwrap().clone();
        let _mysql_pool = app_state.mysql_pool();

        // TODO: check token
        Ok(Authentication(UserContext {
            username: "test".to_string(),
            user_id: 1,
            app_state,
        }))
    }
}
