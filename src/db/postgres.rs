use diesel::result::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::r2d2::{ConnectionManager, Pool};

use crate::db::models::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("resources/migrations/");

pub fn establish_connection(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("Migrations successfully completed."),
        Err(err) => eprintln!("Error running migrations: {:?}", err),
    };

    pool
}

// pub fn search_item(connection: &mut PgConnection, key: &String) {
//     use crate::db::schema::conversations::dsl::*;

//     let results = conversations
//         .filter(id.eq(key))
//         .limit(5)
//         .select(Item::as_select())
//         .load(connection)
//         .expect("Error loading posts");

//     println!("Displaying {} posts", results.len());
//     for item in results {
//         println!("{}", item.id);
//         println!("-----------\n");
//         println!("{}", item.title);
//     }
// }

pub fn create_item(conn: &mut PgConnection, new_item: &Item) -> Result<Item, Error> {
    use crate::db::schema::conversations;

    diesel::insert_into(conversations::table)
        .values(new_item)
        .returning(Item::as_returning())
        .get_result(conn)
}
