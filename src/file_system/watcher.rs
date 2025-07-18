use std::path::{Path, PathBuf};
use std::time::Duration;
use notify::{
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
    Result as NotifyResult,
    Event as NotifyEvent,
    Config as NotifyConfig,
};
use tokio::sync::mpsc;
use tokio::time;
use crate::ui::file_pane::file_pane::NavigatedEvent;

pub struct FileWatcher {
    pub _sender: mpsc::Sender<NavigatedEvent>,
    _watcher: RecommendedWatcher,
    current_path: PathBuf,
}

impl FileWatcher {
    pub fn new(sender: &mpsc::Sender<NavigatedEvent>, path: &Path) -> NotifyResult<Self> {
        let (notify_tx, mut notify_rx) = mpsc::unbounded_channel::<NotifyResult<NotifyEvent>>();

        let cb_tx = notify_tx.clone();
        let config = NotifyConfig::default().with_poll_interval(Duration::from_millis(200));
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = cb_tx.send(res);
            },
            config,
        )?;

        watcher.watch(path, RecursiveMode::NonRecursive)?;

        let pane_sender = sender.clone();

        tokio::spawn(async move {
            let debounce = Duration::from_millis(100);
            let mut pending = false;

            loop {
                tokio::select! {
                    maybe_evt = notify_rx.recv() => {
                        match maybe_evt {
                            Some(Ok(_evt)) => {
                                pending = true;
                            }
                            Some(Err(err)) => {
                                eprintln!("[file-watcher] error: {err}");
                                pending = true;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    _ = time::sleep(debounce), if pending => {
                        if pane_sender.send(NavigatedEvent::FilesUpdated).await.is_err() {
                            break;
                        }
                        pending = false;
                    }
                }
            }
        });

        Ok(Self {
            _sender: sender.clone(),
            _watcher: watcher,
            current_path: path.to_path_buf(),
        })
    }

    pub fn watch_path(&mut self, path: &Path) -> NotifyResult<()> {
        if path == self.current_path {
            return Ok(());
        }

        self._watcher.unwatch(&self.current_path)?;

        self._watcher.watch(path, RecursiveMode::NonRecursive)?;

        self.current_path = path.to_path_buf();
        Ok(())
    }
}