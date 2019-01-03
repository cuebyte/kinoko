use futures::executor::{self, ThreadPool};
use futures::future::Future;
use futures::stream::Stream;
use notify::{self, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::io::{Error, ErrorKind, Result};
use std::pin::Pin;
use std::sync::mpsc::{self, TryRecvError};
use std::task::{LocalWaker, Poll};

pub struct RawEventsFuture {
    tx: mpsc::Sender<RawEvent>,
    rx: mpsc::Receiver<RawEvent>,
    watcher: notify::fsevent::FsEventWatcher,
}

impl RawEventsFuture {
    pub fn new(paths: &[&str]) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut watcher = raw_watcher(tx.clone()).unwrap();
        for path in paths {
            println!("{}", path);
            watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
        }
        RawEventsFuture { tx, rx, watcher }
    }

    // pub fn run(&mut self, ) {
    //     for path in paths {
    //         self.watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    //     }
    // }
}

impl Stream for RawEventsFuture {
    type Item = RawEvent;
    fn poll_next(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Option<Self::Item>> {
        lw.wake();
        match self.rx.try_recv() {
            Ok(event) => {
                Poll::Ready(Some(event))
            }
            Err(err) => match err {
                TryRecvError::Empty => {
                    Poll::Pending
                }
                TryRecvError::Disconnected => {
                    Poll::Ready(None)
                }
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::{self, ThreadPool};
    use futures::prelude::StreamExt;
    use std::process::Command;
    use std::thread;

    #[test]
    fn write_lines() {
        let path = "/Users/william/w/kinoko/crawler.log";
        thread::spawn(move || {
            for _ in 0..10 {
                thread::sleep(std::time::Duration::from_millis(100));
                Command::new("sh")
                    .arg("-c")
                    .arg("echo 0 >> /Users/william/w/kinoko/crawler.log")
                    .spawn()
                    .unwrap();
            }
        });
        thread::spawn(move || {
            executor::block_on(
                async {
                    let mut stream = RawEventsFuture::new(&[&path]);
                    loop {
                        match await!(stream.next()) {
                            Some(event) => println!("{:?}", event),
                            None => println!("..."),
                        }
                    }
                },
            );
        });
        thread::sleep(std::time::Duration::from_secs(4));
        // for _ in 0..10 {
        //     f.write_all(b"abc\n").unwrap();
        //     thread::sleep(Duration::from_millis(1000));
        // }
    }
}
