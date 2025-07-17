use egui::*;
use log::info;
use crate::views::double_pane::double_pane::DoublePane;

mod views;
mod file_system;
mod model;

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async_main())
}

async fn async_main() -> Result<(), eframe::Error> {
    env_logger::init();
    info!("Starting Caesar commander");
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "caesar-commander",
        options,
        Box::new(|_cc| {
            Ok(Box::new(Commander {
                double_pane: DoublePane::new()
            }))
        }),
    )
}

struct Commander {
    double_pane: DoublePane
}


impl eframe::App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {

            self.double_pane.view.ui(ui);

            ui.separator();
        });
    }
}