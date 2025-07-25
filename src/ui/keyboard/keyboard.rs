use egui::{Key, Ui};

use crate::model::{ItemType, MoveDirection};
use crate::ui::file_pane::view::{View, NavigatedEvent};

pub fn handle(view: &mut View, ui: &mut Ui, focused: bool) {
    if !focused {
        return;
    }

    let _ = handle_backspace(view, ui)
        | handle_enter(view, ui)
        | handle_arrow_up(view, ui)
        | handle_arrow_down(view, ui)
        | handle_page_up(view, ui)
        | handle_page_down(view, ui);
}

fn handle_backspace(view: &mut View, ui: &mut Ui) -> bool {
    if ui.input(|i| i.key_pressed(Key::Backspace)) {
        let _ = view.sender().try_send(NavigatedEvent::TraversedUp);
        true
    } else {
        false
    }
}

fn handle_enter(view: &mut View, ui: &mut Ui) -> bool {
    if ui.input(|i| i.key_pressed(Key::Enter)) {
        if let Some(item) = view.get_cursor_item()
            .filter(|it| it.item_type == ItemType::Directory) {
            let _ = view.sender().try_send(NavigatedEvent::DirectoryOpened(item.path.clone()));
        }
        true
    } else {
        false
    }
}

fn handle_arrow_down(view: &mut View, ui: &mut Ui) -> bool {
    if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
        move_cursor(view, ui, 1, MoveDirection::Down);
        true
    } else {
        false
    }
}

fn handle_arrow_up(view: &mut View, ui: &mut Ui) -> bool {
    if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
        move_cursor(view, ui, -1, MoveDirection::Up);
        true
    } else {
        false
    }
}

fn handle_page_down(view: &mut View, ui: &mut Ui) -> bool {
    if ui.input(|i| i.key_pressed(Key::PageDown)) {
        navigate(view, View::page_step(ui));
        true
    } else {
        false
    }
}

fn handle_page_up(view: &mut View, ui: &mut Ui) -> bool {
    if ui.input(|i| i.key_pressed(Key::PageUp)) {
        navigate(view, -View::page_step(ui));
        true
    } else {
        false
    }
}

fn move_cursor(view: &mut View, ui: &Ui, delta: isize, direction: MoveDirection) {
    let len = view.item_count();
    if len == 0 { return; }

    let new_index = ((view.cursor_index() as isize) + delta)
        .clamp(0, (len - 1) as isize) as usize;

    let shift = ui.input(|i| i.modifiers.shift);
    let ctrl  = ui.input(|i| i.modifiers.ctrl);

    let _ = view.sender().try_send(NavigatedEvent::SelectionMoved {
        index: new_index,
        selection: shift,
        additive: ctrl,
        direction: Some(direction),
    });
}

fn navigate(view: &mut View, amount: isize) {
    let len = view.item_count() as isize;
    if len == 0 { return; }

    let current_index = view.cursor_index() as isize;
    let new_index = (current_index + amount).rem_euclid(len) as usize;

    let _ = view.sender().try_send(NavigatedEvent::SelectionMoved {
        index: new_index,
        selection: false,
        additive: false,
        direction: None,
    });
}