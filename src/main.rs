use std::path::PathBuf;
use std::sync::mpsc as std_mpsc;
use std::time::Duration;
use eframe::App;
use egui::*;
use log::info;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::task;
use crate::cli::cli::Cli;
use crate::views::double_pane::double_pane::DoublePane;
use notify::{
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
    Result as NotifyResult,
    Event as NotifyEvent,
};

mod views;
mod file_system;
mod model;
mod cli;

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async_main())
}

async fn async_main() -> Result<(), eframe::Error> {
    env_logger::init();
    info!("Starting Caesar commander");

    let (left_path, right_path) = Cli::parse_and_paths();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "caesar-commander",
        options,
        Box::new(|_cc| {
            let app = Commander::new(left_path, right_path);
            Ok(Box::new(app))
        }),
    )
}

pub struct Commander {
    double_pane: DoublePane,
    fs_rx: UnboundedReceiver<NotifyResult<NotifyEvent>>,
    _fs_watcher: RecommendedWatcher,
}

impl Commander {
    pub fn new(left: PathBuf, right: PathBuf) -> Self {
        let (std_tx, std_rx) = std_mpsc::channel();

        let config = notify::Config::default()
            .with_poll_interval(Duration::from_millis(200));
        let mut watcher: RecommendedWatcher =
            Watcher::new(std_tx, config).expect("failed to init watcher");

        watcher
            .watch(&left, RecursiveMode::NonRecursive)
            .expect("failed to watch left pane dir");
        watcher
            .watch(&right, RecursiveMode::NonRecursive)
            .expect("failed to watch right pane dir");

        let (fs_tx, fs_rx) = unbounded_channel();

        task::spawn_blocking(move || {
            while let Ok(event) = std_rx.recv() {
                let _ = fs_tx.send(event);
            }
        });

        Commander {
            double_pane: DoublePane::new(left, right),
            fs_rx,
            _fs_watcher: watcher,
        }
    }
}



impl App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut changed = false;
        while let Ok(Ok(_)) = self.fs_rx.try_recv() {
            changed = true;
        }

        if changed {
            self.double_pane.refresh();
            ctx.request_repaint();
        }

        CentralPanel::default().show(ctx, |ui| {
            self.double_pane.view.ui(ui);
            ui.separator();
        });
    }
}