use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::db::models::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("resources/migrations/");

pub fn establish_connection(database_url: &str) -> PgConnection {
    let mut conn = PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("Migrations successfully completed."),
        Err(err) => eprintln!("Error running migrations: {:?}", err),
    };

    conn
}

pub fn search_item(connection: &mut PgConnection) {
    use crate::db::schema::posts::dsl::*;

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}