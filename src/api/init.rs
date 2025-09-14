use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[post("/init")]
pub async fn init_db_endpoint(app: web::Data<AppState>) -> impl Responder {
    let mut conn = app.get_connection().unwrap();

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {
            app.set_init_completed(true);
            HttpResponse::Ok().body("Database initialized successfully")}
            ,
        Err(e) => {
            println!("Error running migrations: {}", e);
            HttpResponse::InternalServerError().body(format!("Error running migrations: {}", e))
        }
    }
}
