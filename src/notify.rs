use futures::stream::Stream;
use notify::{self, raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::pin::Pin;
use std::sync::mpsc::{self as sync_mpsc, TryRecvError};
use std::task::{LocalWaker, Poll};
use std::io::{Result, Error, ErrorKind};


pub struct RawEventsFuture {
    rx: sync_mpsc::Receiver<RawEvent>,
    _watcher: LocalWatcher,
}

impl RawEventsFuture {
    pub fn new(paths: &[&str]) -> Result<Self> {
        let (tx, rx) = sync_mpsc::channel();
        let mut _watcher = raw_watcher(tx).unwrap();
        for path in paths {
            if let Err(e) = _watcher.watch(path, RecursiveMode::NonRecursive) {
                return Err(Error::new(ErrorKind::NotFound, e))
            }
        }
        Ok(RawEventsFuture { rx, _watcher })
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

/// The `LocalWatcher` implementation for the current platform
#[cfg(target_os = "linux")]
type LocalWatcher = notify::inotify::INotifyWatcher;
#[cfg(target_os = "macos")]
type LocalWatcher = notify::fsevent::FsEventWatcher;
#[cfg(target_os = "windows")]
type LocalWatcher = notify::windows::ReadDirectoryChangesWatcher;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
type LocalWatcher = notify::poll::PollWatcher;

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor;
    use futures::prelude::StreamExt;
    use std::process::Command;
    use std::thread;

    #[test]
    fn write_lines() {
        let path = "crawler.log";
        std::fs::File::create(path).unwrap();
        thread::spawn(move || {
            executor::block_on(
                async {
                    let mut stream = RawEventsFuture::new(&[&path]).unwrap();
                    loop {
                        match await!(stream.next()) {
                            Some(event) => println!("{:?}", event),
                            None => println!("..."),
                        }
                    }
                },
            );
        });

        for _ in 0..10 {
            thread::sleep(std::time::Duration::from_millis(100));
            Command::new("sh")
                .arg("-c")
                .arg(format!("echo 0 >> {}", path))
                .spawn()
                .unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        std::fs::remove_file(path).unwrap();
    }
}
