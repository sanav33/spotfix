use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, ErrorKind};

#[derive(Debug)]
pub struct BlacklistState {
    pub uris: HashSet<String>,
}

impl BlacklistState {
    pub fn new() -> io::Result<Self> {
        match File::open("./blacklist.txt") {
            Ok(ref mut f) => {
                let mut buf = String::new();
                f.read_to_string(&mut buf)?;
                let uris: HashSet<_> = buf.split("\n").map(|uri| uri.to_string()).collect();
                Ok(BlacklistState { uris })
            }
            Err(error) => {
                if error.kind() == ErrorKind::NotFound {
                    File::create("./blacklist.txt")?;
                    Ok(BlacklistState {
                        uris: HashSet::new(),
                    })
                } else {
                    Err(error)
                }
            }
        }
    }
    // TODO: Add support for adding/removing tracks by track and artist name.
    #[allow(unused)]
    pub fn add_to_blacklist_name_artist(&mut self, name: &str, artist: &str) -> io::Result<()> {
        unimplemented!();
        let mut bl = OpenOptions::new()
            .append(true)
            .create(true)
            .open("blacklist.txt")?;
        bl.write(format!("{} - {}", name, artist).as_bytes())?;
        Ok(())
    }

    /// Recommended
    pub fn add_to_blacklist_uri(&mut self, uri: &str) -> io::Result<()> {
        let mut bl = OpenOptions::new()
            .append(true)
            .create(true)
            .open("./blacklist.txt")?;
        let uri = uri.to_string();
        bl.write(format!("{}\n", uri).as_bytes())?;
        self.uris.insert(uri);
        Ok(())
    }

    /// Recommended
    pub fn remove_from_blacklist_uri(&mut self, uri: &str) -> io::Result<()> {
        self.uris.remove(uri);
        let mut bl = OpenOptions::new()
            .create(true)
            .write(true)
            .open("./blacklist.txt")?;
        let uris: Vec<_> = self.uris.iter().map(|x| x.to_string()).collect();
        bl.write(uris.join("\n").as_bytes())?;
        Ok(())
    }
}
