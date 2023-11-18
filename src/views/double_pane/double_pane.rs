use crate::file_system::navigator::Navigator;
use crate::views::double_pane::double_pane_view::DoublePaneView;
use crate::views::file_pane::file_pane::FilePane;

pub struct DoublePane {
    pub view: DoublePaneView,
}

impl Default for DoublePane {
    fn default() -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let path = home_dir.join("commander-tmp");

        let left_pane = FilePane::new(Navigator::new(&path));
        let right_pane = FilePane::new(Navigator::new(&path));

        DoublePane {
            view: DoublePaneView {
                left_file_pane: left_pane,
                right_file_pane: right_pane,
                focus_state: FocusState::LeftPane,
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