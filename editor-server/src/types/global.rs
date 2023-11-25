use std::sync::Arc;

use crate::server::states::AppState;

#[derive(Debug)]
pub struct UserContext {
    pub username: String,
    pub user_id: i32,
    pub app_state: Arc<AppState>,
}

// impl Drop for UserContext {
//     fn drop(&mut self) {
//         let app_state = Arc::clone(&self.app_state);
//         let user_id = self.user_id;
//
//         tokio::spawn(async move {
//             let mysql = app_state.mysql_pool();
//             #[allow(unused)]
//             let redis = app_state.redis_client();
//
//             let _ = sqlx::query!(
//                 r#"
//                     UPDATE EditingControlFrame SET frame_id = NULL
//                     WHERE user_id = ?;
//                 "#,
//                 user_id
//             )
//             .execute(mysql)
//             .await;
//
//             let _ = sqlx::query!(
//                 r#"
//                     UPDATE EditingPositionFrame SET frame_id = NULL
//                     WHERE user_id = ?;
//                 "#,
//                 user_id
//             )
//             .execute(mysql)
//             .await;
//
//             let _ = sqlx::query!(
//                 r#"
//                     UPDATE EditingLEDEffect SET led_effect_id = NULL
//                     WHERE user_id = ?;
//                 "#,
//                 user_id
//             )
//             .execute(mysql)
//             .await;
//         });
//     }
// }
