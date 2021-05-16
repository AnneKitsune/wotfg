use crate::*;

pub fn entity_curses_render_system(
    cursor: &MapCursor,
    positions: &Components<Position>,
    rendered: &Components<Rendered>,
    render: &RenderInfo,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    let mut cloned = vec![];
    for (pos, rend) in join!(&positions && &rendered) {
        let pos = pos.unwrap();
        let rend = rend.unwrap();
        cloned.push((pos.clone(), rend.clone()));
    }
    cloned.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));
    for (pos, rend) in cloned {
        if pos.chunk_x() == cursor.0.chunk_x()
            && pos.chunk_y() == cursor.0.chunk_y()
            && pos.z() == cursor.0.z()
        {
            if let Some((screen_x, screen_y)) =
                render.position_to_main_area(cursor, (pos.x() as u32, pos.y() as u32))
            {
                curses.move_rc(screen_y as i32, screen_x as i32);
                curses.set_color_pair(rend.color);
                curses.print_char(rend.render_char);
            }
        }
    }
    Ok(())
}
