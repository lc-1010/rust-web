pub mod server;
pub mod handler;
pub mod router;

use server::Server;
fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}
