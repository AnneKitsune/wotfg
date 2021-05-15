use crate::*;

pub fn curses_end_draw_system(curses: &mut Option<Curses>) -> SystemResult {
    // Render
    curses.as_mut().unwrap().0.refresh();
    Ok(())
}

