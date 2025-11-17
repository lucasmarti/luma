use flo_draw::{
    binding::{bind, BindRef},
    create_drawing_window_with_events, with_2d_graphics, DrawEvent, MousePointer, PointerAction,
    WindowProperties,
};
use futures::{executor, StreamExt};

use crate::gui::{game::Game, ui_element::CanvasCoordinate};

pub fn main() {
    let mut window_properties = WindowProperties::from(&"luma Chess");
    window_properties.mouse_pointer = BindRef::from(bind(MousePointer::SystemDefault));
    with_2d_graphics(move || {
        let (canvas, mut events) = create_drawing_window_with_events(window_properties);
        let mut game: Game = Game::new(canvas);
        game.draw();
        executor::block_on(async move {
            while let Some(evt) = events.next().await {
                if let DrawEvent::Pointer(PointerAction::ButtonDown, _, state) = evt {
                    if let Some(coord) = state.location_in_canvas {
                        game.handle_click_event(CanvasCoordinate {
                            x: coord.0 as f32,
                            y: coord.1 as f32,
                        });
                    }
                }
            }
        });
    });
}
mod engine;
mod gui;
