use egui::*;
use crate::model::*;
use crate::views::file_pane::model::FilePane;

impl FilePane {
    pub fn ui(&mut self, ui: &mut Ui) {
        Grid::new("file-views-1")
            .num_columns(self.columns.len())
            .striped(true)
            .show(ui, |ui| {

                // Headers
                for col in &self.columns {
                    ui.label(&col.name);
                }
                ui.end_row();

                // Items
                for item in &mut self.items {
                    match item.item_type {
                        ItemType::File => ui.label("📄"),
                        ItemType::Directory => ui.label("📁"),
                    };

                    ui.label(&item.name);

                    ui.label(format!("{} bytes", item.size));

                    ui.label(&item.modified.to_rfc2822());

                    ui.end_row();
                }


            });
    }
}