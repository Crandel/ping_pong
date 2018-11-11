extern crate ping_pong;
use ping_pong::manager::Manager;

fn main() {
    let mut manager = Manager::new();
    manager.start().unwrap();
}
