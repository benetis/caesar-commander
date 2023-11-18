use crate::file_system::navigator::Navigator;
use crate::views::file_pane::file_pane_view::FilePaneView;

pub struct FilePane {
    pub view: FilePaneView,
    pub navigator: Navigator,
}

impl FilePane {
    pub fn new(navigator: &Navigator) -> Self {
        let view = FilePaneView::default();
        let items = navigator.list_contents();

        Self {
            view: FilePaneView {
                items,
                ..view
            },
            navigator: navigator.clone(),
        }
    }

}