use crate::*;

pub fn curses_update_render_info_system(
    curses: &Option<Curses>,
    render: &mut RenderInfo,
) -> SystemResult {
    let (screen_height, screen_width) = curses.as_ref().unwrap().0.get_row_col_count();
    let (screen_height, screen_width) = (screen_height as u32, screen_width as u32);
    render.screen_width = screen_width;
    render.screen_height = screen_height;
    Ok(())
}
