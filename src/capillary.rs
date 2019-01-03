use futures::stream::Stream;
use notify::{self, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::pin::Pin;
use std::sync::mpsc::{self, TryRecvError};
use std::task::{LocalWaker, Poll};

pub struct RawEventsFuture {
    rx: mpsc::Receiver<RawEvent>,
    _watcher: notify::fsevent::FsEventWatcher,
}

impl RawEventsFuture {
    pub fn new(paths: &[&str]) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut _watcher = raw_watcher(tx).unwrap();
        for path in paths {
            _watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
        }
        RawEventsFuture { rx, _watcher }
    }
}

impl Stream for RawEventsFuture {
    type Item = RawEvent;
    fn poll_next(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Option<Self::Item>> {
        lw.wake();
        match self.rx.try_recv() {
            Ok(event) => Poll::Ready(Some(event)),
            Err(err) => match err {
                TryRecvError::Empty => Poll::Pending,
                TryRecvError::Disconnected => Poll::Ready(None),
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor;
    use futures::prelude::StreamExt;
    use std::process::Command;
    use std::thread;

    #[test]
    fn write_lines() {
        let path = "/Users/william/w/kinoko/crawler.log";
        thread::spawn(move || {
            executor::block_on(async {
                let mut stream = RawEventsFuture::new(&[&path]);
                loop {
                    match await!(stream.next()) {
                        Some(event) => println!("{:?}", event),
                        None => println!("..."),
                    }
                }
            });
        });

        for _ in 0..10 {
            thread::sleep(std::time::Duration::from_millis(100));
            Command::new("sh")
                .arg("-c")
                .arg("echo 0 >> /Users/william/w/kinoko/crawler.log")
                .spawn()
                .unwrap();
        }
    }
}
