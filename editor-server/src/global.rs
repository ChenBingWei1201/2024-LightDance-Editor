use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::server::states::AppState;

pub static APP_STATE: OnceCell<Arc<AppState>> = OnceCell::new();
