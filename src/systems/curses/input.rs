use crate::*;

pub fn curses_input_system(
    keymap: &Keymap,
    input_ev: &mut Vec<InputEvent>,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    while let Some(input) = curses.get_input() {
        if let Some(ev) = keymap.map.get(&input) {
            let hanged = input_ev
                .iter()
                .flat_map(|e| {
                    if let InputEvent::Hanged(h) = e {
                        Some(h)
                    } else {
                        None
                    }
                })
                .next();
            let had_hanged_input = hanged.is_some();
            if let Some(hanged) = hanged {
                match hanged {
                    HangedInput::Mine => match ev {
                        InputEvent::PlayerAction(PlayerAction::MoveLeft) => input_ev.push(
                            InputEvent::PlayerAction(PlayerAction::Mine(Direction::West)),
                        ),
                        InputEvent::PlayerAction(PlayerAction::MoveRight) => input_ev.push(
                            InputEvent::PlayerAction(PlayerAction::Mine(Direction::East)),
                        ),
                        InputEvent::PlayerAction(PlayerAction::MoveUp) => input_ev.push(
                            InputEvent::PlayerAction(PlayerAction::Mine(Direction::North)),
                        ),
                        InputEvent::PlayerAction(PlayerAction::MoveDown) => input_ev.push(
                            InputEvent::PlayerAction(PlayerAction::Mine(Direction::South)),
                        ),
                        _ => {}
                    },
                    HangedInput::MacroRecord => {
                        // TODO
                    }
                    HangedInput::MacroReplay => {}
                }
            } else {
                input_ev.push(*ev);
            }
            if had_hanged_input {
                input_ev.retain(|e| {
                    if let InputEvent::Hanged(_) = e {
                        false
                    } else {
                        true
                    }
                });
            }
        }
    }
    Ok(())
}
