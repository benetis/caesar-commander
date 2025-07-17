use crate::cli::cli::Cli;
use crate::views::double_pane::double_pane::DoublePane;
use eframe::{App, NativeOptions};
use egui::*;
use log::info;

mod cli;
mod file_system;
mod model;
mod views;

fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async_main())
}

async fn async_main() -> Result<(), eframe::Error> {
    env_logger::init();
    info!("Starting Caesar commander");

    let params = Cli::new();

    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(vec2(1024.0, 768.0))
            .with_min_inner_size(vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "caesar-commander",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(params.scale);

            let app = Commander {
                double_pane: DoublePane::new(params.left_path, params.right_path),
            };

            Ok(Box::new(app))
        }),
    )
}

pub struct Commander {
    double_pane: DoublePane,
}

impl App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.double_pane.view.ui(ui);
            ui.separator();
        });
    }
}
