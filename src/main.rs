use egui::*;
use log::info;
use crate::file_system::navigator::Navigator;
use crate::views::file_pane::file_pane::FilePane;
use crate::views::file_pane::file_pane_view::FilePaneView;

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
            let navigator = Navigator::default();
            let file_pane = FilePane::new(navigator.clone());

            Box::new(Commander {
                file_pane,
                navigator,
            })
        }),
    )
}

struct Commander {
    file_pane: FilePane,
    navigator: Navigator,
}


impl eframe::App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.file_pane.view.ui(ui);

            ui.separator();
        });
    }
}