mod database;
mod error;
mod model;
mod server;

use database::Database;
use server::Server;
fn main() {
    env_logger::init();
    let db = Database::new();
    let mut server = Server::new(db, "8080");
    if let Err(err) = server.start() {
        log::error!("{err:#?}");
    }
}
