use crate::*;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputEvent {
    ViewUp,
    ViewDown,
    ViewLeft,
    ViewRight,
    ViewLayerUp,
    ViewLayerDown,
    Cancel,
    Accept,
    Hanged(HangedInput),
    PlayerAction(PlayerAction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveLayerUp,
    MoveLayerDown,
}

#[derive(Clone, Default, Debug)]
pub struct PlayerActionQueue {
    pub queue: VecDeque<PlayerAction>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keymap {
    pub map: HashMap<Input, InputEvent>,
}

impl Default for Keymap {
    fn default() -> Self {
        Keymap {
            map: [
                (Input::Character('w'), InputEvent::PlayerAction(PlayerAction::MoveUp)),
                (Input::Character('a'), InputEvent::PlayerAction(PlayerAction::MoveLeft)),
                (Input::Character('s'), InputEvent::PlayerAction(PlayerAction::MoveDown)),
                (Input::Character('d'), InputEvent::PlayerAction(PlayerAction::MoveRight)),
                (Input::Character('q'), InputEvent::PlayerAction(PlayerAction::MoveLayerDown)),
                (Input::Character('e'), InputEvent::PlayerAction(PlayerAction::MoveLayerUp)),
                (Input::Character('W'), InputEvent::ViewUp),
                (Input::Character('A'), InputEvent::ViewLeft),
                (Input::Character('S'), InputEvent::ViewDown),
                (Input::Character('D'), InputEvent::ViewRight),
                (Input::Character('Q'), InputEvent::ViewLayerDown),
                (Input::Character('E'), InputEvent::ViewLayerUp),
                (Input::Character('m'), InputEvent::Hanged(HangedInput::Mine)),
                (Input::Character('\n'), InputEvent::Accept),
                (Input::Character('\u{1b}'), InputEvent::Cancel), // Escape
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}
