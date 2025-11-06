//! File watching for live coding

use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct FileWatcher {
    _watcher: notify::RecommendedWatcher,
    receiver: Receiver<notify::Result<Event>>,
}

impl FileWatcher {
    /// Create a new file watcher for the given path
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
        })
    }

    /// Wait for the next file modification event
    /// Returns true if file was modified, false on timeout
    pub fn wait_for_change(&self, timeout: Duration) -> bool {
        match self.receiver.recv_timeout(timeout) {
            Ok(Ok(event)) => matches!(
                event.kind,
                EventKind::Modify(_) | EventKind::Create(_)
            ),
            _ => false,
        }
    }

    /// Check if file changed (non-blocking)
    pub fn poll_change(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(Ok(event)) => matches!(
                event.kind,
                EventKind::Modify(_) | EventKind::Create(_)
            ),
            _ => false,
        }
    }
}
