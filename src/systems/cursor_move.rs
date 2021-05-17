use crate::*;

pub fn cursor_move_system(input_ev: &mut Vec<InputEvent>, cursor: &mut MapCursor) -> SystemResult {
    for ev in input_ev {
        let new = match ev {
            InputEvent::ViewUp => cursor.0.move_towards(Direction::North),
            InputEvent::ViewDown => cursor.0.move_towards(Direction::South),
            InputEvent::ViewRight => cursor.0.move_towards(Direction::East),
            InputEvent::ViewLeft => cursor.0.move_towards(Direction::West),
            InputEvent::ViewLayerUp => cursor.0.move_towards(Direction::Up),
            InputEvent::ViewLayerDown => cursor.0.move_towards(Direction::Down),
            _ => continue,
        };
    }
    Ok(())
}
