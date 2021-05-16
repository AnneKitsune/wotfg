use crate::*;

pub fn player_move_system(actions: &Vec<PlayerAction>, players: &Components<Player>,
    chunks: &Vec<Chunk>,
    positions: &mut Components<Position>) -> SystemResult {
    for ev in actions {
        for (player, mut position) in join!(&players && &mut positions) {
            let position = &mut position.as_mut().unwrap();
            // TODO check that action comes from right player
            // TODO check collision
            // TODO check if we are on stairs/ramp
            match ev {
                PlayerAction::MoveUp => position.move_towards(Direction::North),
                PlayerAction::MoveDown => position.move_towards(Direction::South),
                PlayerAction::MoveRight => position.move_towards(Direction::East),
                PlayerAction::MoveLeft => position.move_towards(Direction::West),
                PlayerAction::MoveLayerUp => position.move_towards(Direction::Up),
                PlayerAction::MoveLayerDown => position.move_towards(Direction::Down),
                _ => continue,
            }
        }
    }
    Ok(())
}
