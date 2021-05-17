use crate::*;

pub fn center_cursor_system(
    input_ev: &Vec<InputEvent>,
    players: &Components<Player>,
    positions: &Components<Position>,
    auth: &Auth,
    cursor: &mut MapCursor,
) -> SystemResult {
    for ev in input_ev {
        match ev {
            InputEvent::CenterCursor => {
                for (player, position) in join!(&players && &positions) {
                    if player.unwrap().id == auth.id {
                        cursor.0 = position.unwrap().clone();
                    }
                }
            },
            _ => {},
        }
    }
    Ok(())
}
