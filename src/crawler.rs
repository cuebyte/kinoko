use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;
use std::{thread, time};

pub struct Crawler {}

impl Crawler {
    pub fn new() -> Self {
        Crawler {}
    }

    pub fn crawl(path: &str) {
        let mut f = OpenOptions::new().read(true).open(path).unwrap();
        let mut x = 0;
        loop {
            thread::sleep(time::Duration::from_millis(2));
            let xx = f.metadata().unwrap().len();
            if x != xx {
                x = xx;
                println!("!{}", x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::{thread, time};

    #[test]

    fn write_lines() {
        let path = "crawler.log";
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        thread::spawn(move || {
            Crawler::crawl(path);
        });
        for _ in 0..10 {
            f.write_all(b"abc\n").unwrap();
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
