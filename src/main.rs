use flo_draw::{
    binding::{bind, BindRef},
    create_drawing_window_with_events, with_2d_graphics, DrawEvent, MousePointer, PointerAction,
    WindowProperties,
};
use futures::{executor, StreamExt};

use crate::gui::chess_board_canvas::ChessBoardCanvas;

pub fn main() {
    with_2d_graphics(|| {
        let mut window_properties = WindowProperties::from(&"luma Chess");
        window_properties.mouse_pointer = BindRef::from(bind(MousePointer::SystemDefault));
        let (canvas, events) = create_drawing_window_with_events(window_properties);
        let mut chess_board_canvas: ChessBoardCanvas = ChessBoardCanvas::new(canvas);
        chess_board_canvas.draw();

        executor::block_on(async move {
            let mut events = events;
            while let Some(evt) = events.next().await {
                if let DrawEvent::Pointer(PointerAction::ButtonDown, _, state) = evt {
                    chess_board_canvas.handle_click_event(state.location_in_canvas);
                }
            }
        });
    });
}
mod engine;
mod gui;
