use handlers::start_server;

mod env;
mod handlers;
mod repl;
mod utils;

fn main() {
    start_server().unwrap();
}
