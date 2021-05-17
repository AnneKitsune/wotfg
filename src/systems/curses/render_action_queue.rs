use crate::*;

pub fn curses_render_action_queue_system(
    render: &RenderInfo,
    actions: &PlayerActionQueue,
    keymap: &Keymap,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(render.screen_height as i32, 0);

    let mut x = 6;
    for action in actions.queue.iter() {
        let c = match action {
            PlayerAction::Mine(_) => keymap.key_for_event(&InputEvent::Hanged(HangedInput::Mine)).unwrap(),
            a => keymap.key_for_event(&InputEvent::PlayerAction(*a)).expect("No key associated to one of the PlayerAction. curses_render_action_queue_system needs to be updated."),
        };
        if c != ' ' {
            curses.move_rc(render.screen_height as i32, x);
            curses.print_char(c);
            x += 1;
        }
    }
    curses.move_rc(render.screen_height as i32, 0);
    curses.print_char('|');
    Ok(())
}
