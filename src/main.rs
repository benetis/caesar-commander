use egui::*;
use log::info;

pub mod file_pane;

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
        Box::new(|_cc| Box::<Commander>::default()),
    )
}

#[derive(Default)]
struct Commander {
    file_pane: file_pane::FilePane,
}


impl eframe::App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.file_pane.ui(ui);

            ui.separator();
        });
    }
}