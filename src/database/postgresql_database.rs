use crate::database::IDatabase;
use std::collections::HashMap;
use serenity::model::id::UserId;
use tokio_postgres::{NoTls, Socket};
use tokio_postgres::tls::NoTlsStream;
use std::error::Error;
use std::env;

struct PostgreSQLDatabase {
    client: tokio_postgres::Client,
    connection: tokio_postgres::Connection<Socket, NoTlsStream>,
}

impl PostgreSQLDatabase {
    async fn new() -> Self {
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

impl IDatabase for PostgreSQLDatabase {
    async fn connect(&self) {
        if let Err(e) = &self.connection.await {
            eprintln!("Failed to connect to the PostgreSQL database: {}", e);
        }
    }

    async fn list_active_members(&self) -> Vec<MemberMsgCount> {
        todo!()
    }

    async fn commit_msg_count(&self, batch: HashMap<UserId, u32>) {
        todo!()
    }

    async fn get_msg_count(&self, user: UserId) -> u32 {
        todo!()
    }
}