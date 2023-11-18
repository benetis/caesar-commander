use egui::*;
use log::info;
use crate::file_system::navigator::Navigator;
use crate::views::file_pane::file_pane::FilePane;

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
            let left_file_pane = FilePane::new(Navigator::default());
            let right_file_pane = FilePane::new(Navigator::default());

            Box::new(Commander {
                left_file_pane,
                right_file_pane
            })
        }),
    )
}

struct Commander {
    left_file_pane: FilePane,
    right_file_pane: FilePane,
}


impl eframe::App for Commander {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {

            while let Ok(event) = self.left_file_pane.receiver.try_recv() {
                self.left_file_pane.handle_navigation_event(&event);
            }

            while let Ok(event) = self.right_file_pane.receiver.try_recv() {
                self.right_file_pane.handle_navigation_event(&event);
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    self.left_file_pane.view.ui(ui, "left");
                });
                ui.vertical(|ui| {
                    self.right_file_pane.view.ui(ui, "right");
                });
            });


            ui.separator();
        });
    }
}