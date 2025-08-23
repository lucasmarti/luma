use crate::gui::configuration::{Action, MENU_BUTTONS};

pub fn get_action(location_x: f64, location_y: f64) -> Option<Action> {
    for button in MENU_BUTTONS {
        if button.x_min as f64 <= location_x
            && location_x <= button.x_max as f64
            && button.y_min as f64 <= location_y
            && location_y <= button.y_max as f64
        {
            return Some(button.action);
        }
    }
    None
}
