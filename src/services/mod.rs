pub mod buddha;
pub mod discord;

use std::sync::Arc;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub buddha_service: Arc<buddha::BuddhaServiceImpl>,
    pub notify_service: Arc<discord::DiscordNotifyServiceImpl>,
}

impl AppState {
    pub fn new(
        config: Config,
        buddha_service: buddha::BuddhaServiceImpl,
        notify_service: discord::DiscordNotifyServiceImpl,
    ) -> Self {
        Self {
            config: Arc::new(config),
            buddha_service: Arc::new(buddha_service),
            notify_service: Arc::new(notify_service),
        }
    }
}

pub use buddha::BuddhaServiceImpl;
pub use discord::DiscordNotifyServiceImpl;