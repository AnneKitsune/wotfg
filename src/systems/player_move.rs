use crate::*;

pub fn player_move_system(
    actions: &PlayerActionQueue,
    players: &Components<Player>,
    chunks: &HashMap<(u32, u32), Chunk>,
    cursor: &mut MapCursor,
    positions: &mut Components<Position>,
) -> SystemResult {
    if let Some(ev) = actions.queue.front() {
        for (player, mut position) in join!(&players && &mut positions) {
            let position = position.as_mut().unwrap();
            let mut new_position = position.clone();
            // TODO check that action comes from right player
            // TODO check if we are on stairs/ramp
            match ev {
                PlayerAction::MoveUp => new_position.move_towards(Direction::North),
                PlayerAction::MoveDown => new_position.move_towards(Direction::South),
                PlayerAction::MoveRight => new_position.move_towards(Direction::East),
                PlayerAction::MoveLeft => new_position.move_towards(Direction::West),
                PlayerAction::MoveLayerUp => new_position.move_towards(Direction::Up),
                PlayerAction::MoveLayerDown => new_position.move_towards(Direction::Down),
                _ => continue,
            }
            if let Some(chunk) = chunks.get(&(new_position.chunk_x(), new_position.chunk_y())) {
                if !chunk
                    .collisions
                    .get(new_position.z() as usize)
                    .expect("No collision map for loaded chunk.")
                    .is_set(new_position.x() as u32, new_position.y() as u32)
                {
                    // TODO move this to a new system that receives network events of player
                    // moved
                    if **position == cursor.0 {
                        cursor.0 = new_position.clone();
                    }

                    **position = new_position;
                }
            }
        }
    }
    Ok(())
}
