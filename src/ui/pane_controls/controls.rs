use tokio::sync::mpsc;
use crate::model::pane_controls::PaneControlsEvent;
use crate::ui::pane_controls::controls_view::PaneControlsView;

pub struct PaneControls {
    pub view: PaneControlsView,
    pub receiver: mpsc::Receiver<PaneControlsEvent>,
}

impl PaneControls {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1);

        PaneControls {
            view: PaneControlsView {
                sender,
            },
            receiver,
        }
    }
}