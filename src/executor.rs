use crate::blacklist::BlacklistState;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle, sleep};
use osascript;
use std::time::Duration;

pub fn get_skip_current_track(
    bs: Arc<Mutex<BlacklistState>>,
    tx: mpsc::Sender<bool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let tx = tx.clone();
        let script = osascript::JavaScript::new("
            var App = Application('Spotify');
            return App.currentTrack().spotifyUrl();
        ");
        loop {
            let track = script.execute::<String>().unwrap_or_else(|_| std::process::exit(0));
            println!("currently playing: {}", track);
            if bs
                .lock()
                .unwrap()
                .uris
                .contains(&track)
            {
                println!("sending!");
                tx.send(true).unwrap();
            }
            sleep(Duration::from_secs(1));
        }
    })
}

pub fn skip_handler(rx: mpsc::Receiver<bool>) -> JoinHandle<()> {
    thread::spawn(move || {
        let script = osascript::JavaScript::new("
            var App = Application('Spotify');
            App.nextTrack();
        ");
        while let Ok(true) = rx.recv() {
            script.execute::<()>().expect("couldn't skip track");
        }
    })
}
