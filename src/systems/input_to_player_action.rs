use crate::*;

pub fn input_to_player_action(
    inputs: &Vec<InputEvent>,
    actions: &mut PlayerActionQueue,
) -> SystemResult {
    for i in inputs {
        match i {
            InputEvent::PlayerAction(action) => actions.queue.push_back(action.clone()),
            InputEvent::Cancel => actions.queue.clear(),
            _ => {}
        }
    }
    Ok(())
}
