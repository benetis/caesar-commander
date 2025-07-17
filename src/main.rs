use crate::cli::cli::Cli;
use crate::views::double_pane::double_pane::DoublePane;
use eframe::App;
use egui::*;
use log::info;

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
            let app = Commander {
                double_pane: DoublePane::new(left_path, right_path),
            };

            Ok(Box::new(app))
        }),
    )
}

pub struct Commander {
    double_pane: DoublePane
}


impl App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.double_pane.view.ui(ui);
            ui.separator();
        });
    }
}