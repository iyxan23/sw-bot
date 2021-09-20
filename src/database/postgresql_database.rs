use crate::database::{IDatabase, MemberMsgCount};
use std::collections::HashMap;
use serenity::model::id::UserId;
use tokio_postgres::{NoTls, Socket};
use tokio_postgres::tls::NoTlsStream;
use std::env;
use async_trait::async_trait;

pub struct PostgreSQLDatabase {
    client: tokio_postgres::Client,
    connection: tokio_postgres::Connection<Socket, NoTlsStream>,
}

unsafe impl Sync for PostgreSQLDatabase {}
unsafe impl Send for PostgreSQLDatabase {}

impl PostgreSQLDatabase {
    pub async fn new() -> Self {
        let postgres_credentials =
            env::var("POSTGRES_CREDENTIALS")
                .expect("POSTGRES_CREDENTIALS environment variable is not present");

        let (client, connection) =
            tokio_postgres::connect(postgres_credentials.as_str(), NoTls)
                .await
                .expect("Cannot connect to the PostgreSQL database");

        PostgreSQLDatabase {
            client, connection
        }
    }
}

#[async_trait]
impl IDatabase for PostgreSQLDatabase {
    async fn init(&self) {
        // create table(s) if they doesn't exist
        self.client
            .execute("CREATE TABLE IF NOT EXISTS msg_count ( \
                    user_id BIGINT PRIMARY KEY, \
                    count INT NOT NULL DEFAULT '0', \
                )", &[])
            .await
            .expect("Cannot create the table msg_count");
    }

    async fn list_active_members(&self) -> Vec<MemberMsgCount> {
        let mut active_members = Vec::new();

        let rows =
            self.client
                .query("SELECT * FROM msg_count ORDER BY count DESC LIMIT 10", &[])
                .await
                .expect("Failed to prepare the \"list active members\" statement");

        for row in rows {
            let user_id: UserId = {
                let user_id_signed: i64 = row.get(0);
                UserId(user_id_signed as u64)
            };

            let count: i32 = row.get(1);

            active_members.push(MemberMsgCount {
                user_id,
                count: (count as u32)
            });
        }

        return active_members;
    }

    async fn commit_msg_count(&self, batch: HashMap<UserId, u32>) {
        for user_id in batch.keys() {
            let count = (*batch.get(user_id).unwrap()) as i32;

            self.client
                .execute(
                    "UPDATE msg_count (count) VALUES ($1) WHERE user_id = $2",
                    &[&count, &(user_id.0 as i64)]
                )
                .await
                .expect(format!("Failed to update msg_count for {}", user_id.0).as_str());
        }
    }

    async fn get_msg_count(&self, user: UserId) -> u32 {
        let rows = self.client
            .query(
                "SELECT count FROM msg_count WHERE user_id = $1",
                &[&(user.0 as i64)]
            )
            .await
            .unwrap();

        let count_signed: i32 = rows[0].get(0);
        count_signed as u32
    }
}