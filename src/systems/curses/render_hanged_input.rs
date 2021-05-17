use crate::*;

pub fn curses_render_hanged_input_system(
    render: &RenderInfo,
    input_ev: &Vec<InputEvent>,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(render.screen_height as i32, 0);

    for ev in input_ev {
        match ev {
            InputEvent::Hanged(hanged) => {
                let s = match hanged => {
                    HangedInput::Mine => "mine",
                    HangedInput::MacroRecord => "rec",
                    HangedInput::MacroReplay => "play",
                }
                curses.print(s);
            },
            _ => {},
        }
    }
    curses.move_rc(render.screen_height as i32, 0);
    curses.print_char('|');
    Ok(())
}
