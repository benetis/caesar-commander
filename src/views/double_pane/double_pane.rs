use crate::file_system::navigator::Navigator;
use crate::views::double_pane::double_pane_view::DoublePaneView;
use crate::views::file_pane::file_pane::FilePane;

pub struct DoublePane {
    pub view: DoublePaneView,
}

impl Default for DoublePane {
    fn default() -> Self {
        DoublePane {
            view: DoublePaneView {
                left_file_pane: FilePane::new(Navigator::default()),
                right_file_pane: FilePane::new(Navigator::default()),
                focus_state: FocusState::LeftPane
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
}