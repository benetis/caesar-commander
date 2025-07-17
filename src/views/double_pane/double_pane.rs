use crate::file_system::navigator::Navigator;
use crate::views::double_pane::double_pane_view::DoublePaneView;
use crate::views::file_pane::file_pane::FilePane;
use crate::views::pane_controls::controls::PaneControls;
use std::path::PathBuf;

pub struct DoublePane {
    pub view: DoublePaneView
}

impl DoublePane {
    pub fn new(left: PathBuf, right: PathBuf) -> Self {
        let left_pane = FilePane::new(Navigator::new(&left));
        let right_pane = FilePane::new(Navigator::new(&right));

        let pane_controls = PaneControls::new();

        DoublePane {
            view: DoublePaneView {
                left_file_pane: left_pane,
                right_file_pane: right_pane,
                focus_state: FocusState::LeftPane,
                pane_controls,
            }
        }
    }
}

pub enum FocusState {
    LeftPane,
    RightPane,
}

impl FocusState {
    pub fn is_left(&self) -> bool {
        match self {
            FocusState::LeftPane => true,
            FocusState::RightPane => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            FocusState::LeftPane => false,
            FocusState::RightPane => true,
        }
    }

    pub fn next(&mut self) {
        match self {
            FocusState::LeftPane => *self = FocusState::RightPane,
            FocusState::RightPane => *self = FocusState::LeftPane,
        }
    }
}