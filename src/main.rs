use server::start_server;

mod env;
mod handlers;
mod repl;
mod server;
mod utils;

fn main() {
    start_server().unwrap();
}
