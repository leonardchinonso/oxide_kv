mod database;
mod error;
mod server;

use database::Database;
use server::Server;
fn main() {
    env_logger::init();
    let mut db = Database::new();
    let server = Server::new("8080");

    server.start();
}
