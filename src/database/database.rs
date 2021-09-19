use serenity::model::id::UserId;
use std::collections::HashMap;
use async_trait::async_trait;

// I made this so I can switch databases easily and the database-related code & bot-related code are
// separated from each other.
#[async_trait]
pub trait IDatabase {
    async fn init(&self);

    /// ===== Used for stats =====

    /// Lists active members of the server
    async fn list_active_members(&self) -> Vec<MemberMsgCount>;

    /// Used to commit the "batch-counted" messages
    async fn commit_msg_count(&self, batch: HashMap<UserId, u32>);

    /// Used to get a member's msg count
    async fn get_msg_count(&self, user: UserId) -> u32;
}

pub struct MemberMsgCount {
    pub user_id: UserId,
    pub count: u32
}
