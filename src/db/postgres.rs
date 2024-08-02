use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use uuid::Uuid;

use crate::db::models::*;
use crate::db::schema::*;

use tracing::{debug, info, error};

use anyhow::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("resources/migrations/");

pub fn establish_connection(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => info!("Migrations successfully completed."),
        Err(err) => error!("Error running migrations: {:?}", err),
    };

    pool
}

pub fn insert_conversation<'a>(
    pg_conn: &'a mut PgConnection,
    conversation: &'a Conversation,
) -> Result<&'a Conversation, Error> {
    debug!("Inserting conversation: {:?}", conversation.id);
    
    let result = pg_conn.transaction(|conn| {
        let db_conversation: DbConversation = conversation.clone().into();
        debug!("DB conversation: {:?}", db_conversation);

        let conversation_id = diesel::insert_into(conversations::table)
            .values(&db_conversation)
            .returning(conversations::id)
            .get_result::<Uuid>(conn)?;

        debug!("Inserted conversation: {:?}", conversation_id);

        for mapping in conversation.mapping.values() {            
            if let Some(msg) = &mapping.message {
                match insert_message(conn, msg) {
                    Ok(message_id) => debug!("Inserted message ID: {:?}", message_id),
                    Err(e) => error!("Failed to insert message {:?} into conversation {:?}:\n{:?}", msg.id, conversation_id, e),
                }
            }

            match insert_mapping(conn, mapping) {
                Ok(mapping_id) => debug!("Inserted mapping ID: {:?}", mapping_id),
                Err(e) => error!("Failed to insert mapping {:?} into conversation {:?}:\n{:?}", mapping.id, conversation_id, e),
            }
        }

        Ok(conversation)
    });

    match result {
        Ok(conversation) => {
            info!("Transaction succeeded with conversation ID: {:?}", conversation.id);
            Ok(conversation)
        }
        Err(e) => {
            error!("Transaction failed: {:?}", e);
            Err(e)
        }
    }
}

fn insert_mapping(conn: &mut PgConnection, mapping: &Mapping) -> Result<Uuid, Error> {
    let db_mapping: DbMapping = mapping.clone().into();
    match diesel::insert_into(mappings::table)
        .values(&db_mapping)
        .returning(mappings::id)
        .get_result(conn)
    {
        Ok(mapping_id) => Ok(mapping_id),
        Err(e) => Err(Error::msg(e.to_string())),
    }
}

fn insert_message(conn: &mut PgConnection, msg: &Message) -> Result<Uuid, Error> {
    let db_message: DbMessage = msg.clone().into();
    match diesel::insert_into(messages::table)
        .values(&db_message)
        .returning(messages::id)
        .get_result(conn)
    {
        Ok(message_id) => Ok(message_id),
        Err(e) => Err(Error::msg(e.to_string())),
    }
}