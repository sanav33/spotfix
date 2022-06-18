use std::error::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub mod blacklist;
pub mod executor;

use blacklist::BlacklistState;

fn main() -> Result<(), Box<dyn Error>> {
    let bs = Arc::new(Mutex::new(BlacklistState::new()?));
    println!("{:?}", bs.lock().unwrap().uris);
    let (tx, rx) = mpsc::channel();
    let runner = executor::get_skip_current_track(Arc::clone(&bs), tx);
    let skipper = executor::skip_handler(rx);
    runner.join().unwrap();
    skipper.join().unwrap();
    Ok(())
}
