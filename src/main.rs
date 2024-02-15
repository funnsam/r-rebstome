#[macro_use]
mod log;
mod server;

fn main() {
    let config = server::config::Config::read();
    server::Server::new(config).update();
}
