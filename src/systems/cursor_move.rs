use crate::*;

fn cursor_move_system(input_ev: &mut Vec<InputEvent>, cursor: &mut MapCursor) -> SystemResult {
    for ev in input_ev {
        let new = match ev {
            InputEvent::MoveUp => cursor.0.move_towards(Direction::North),
            InputEvent::MoveDown => cursor.0.move_towards(Direction::South),
            InputEvent::MoveRight => cursor.0.move_towards(Direction::East),
            InputEvent::MoveLeft => cursor.0.move_towards(Direction::West),
            InputEvent::LayerUp => cursor.0.move_towards(Direction::Up),
            InputEvent::LayerDown => cursor.0.move_towards(Direction::Down),
            _ => continue,
        };
    }
    Ok(())
}

