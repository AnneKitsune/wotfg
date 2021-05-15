use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputEvent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    LayerUp,
    LayerDown,
    Cancel,
    Accept,
    Hanged(HangedInput),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keymap {
    pub map: HashMap<Input, InputEvent>,
}

impl Default for Keymap {
    fn default() -> Self {
        Keymap {
            map: [
                (Input::Character('h'), InputEvent::MoveLeft),
                (Input::Character('l'), InputEvent::MoveRight),
                (Input::Character('j'), InputEvent::MoveDown),
                (Input::Character('k'), InputEvent::MoveUp),
                (Input::Character(';'), InputEvent::LayerDown),
                (Input::Character('.'), InputEvent::LayerUp),
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

