use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc;
use std::time::Duration;

pub struct Capillary {}

impl Capillary {
    pub fn new() -> Self {
        Capillary {}
    }

    pub fn crawl(path: &str) {
        // Create a channel to receive the events.
        let (tx, rx) = mpsc::channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        // let mut watcher = watcher(tx, Duration::from_millis(200)).unwrap();
        let mut watcher = watcher(tx, Duration::from_millis(200)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        loop {
            println!("!!!!");
            match rx.recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        // let mut f = OpenOptions::new().read(true).open(path).unwrap();
        // let mut x = 0;
        // loop {
        //     thread::sleep(time::Duration::from_millis(2));
        //     let xx = f.metadata().unwrap().len();
        //     if x != xx {
        //         x = xx;
        //         println!("!{}", x);
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command};
    use std::thread;

    #[test]

    fn write_lines() {
        let path = "/Users/william/w/kinoko/crawler.log";

        thread::spawn(move || {
            Capillary::crawl(path);
        });
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(200));
            Command::new("sh")
                .arg("-c")
                .arg("echo 0 >> /Users/william/w/kinoko/crawler.log")
                .spawn()
                .unwrap();
        }

        // for _ in 0..10 {
        //     f.write_all(b"abc\n").unwrap();
        //     thread::sleep(Duration::from_millis(1000));
        // }
    }
}
