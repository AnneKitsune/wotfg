use crate::*;

pub fn curses_render_system(
    cursor: &MapCursor,
    render: &RenderInfo,
    chunks: &HashMap<(u32, u32), Chunk>,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    // Tile space
    // Then screenspace
    // Then borders
    // Then ui

    // ---- Tile Space ----

    // ---- Screen Space ----

    // Clear the screen
    curses.set_color_pair(*COLOR_NORMAL);
    for y in 0..render.screen_height {
        for x in 0..render.screen_width {
            curses.move_rc(y as i32, x as i32);
            curses.print_char(' ');
        }
    }

    if render.screen_height < MAIN_AREA_MARGIN_BOTTOM + MAIN_AREA_MARGIN_TOP + 2
        || render.screen_width < MAIN_AREA_MARGIN_RIGHT + MAIN_AREA_MARGIN_LEFT + 2
    {
        curses.move_rc(0, 0);
        curses.print("Screen too small");
        return Ok(());
    }

    curses.set_color_pair(*COLOR_NORMAL);

    let map_offset = render.map_offsets(cursor);
    let (xmax, ymax) = render.maximum_positions();

    if let Some(chunk) = chunks.get(&(cursor.0.chunk_x(), cursor.0.chunk_y())) {
        // Render the map tiles and border
        for y in MAIN_AREA_MARGIN_TOP..ymax {
            for x in MAIN_AREA_MARGIN_LEFT..xmax {
                // calculated manually here because its faster
                let x_pos = map_offset.0 + x - MAIN_AREA_MARGIN_LEFT;
                let y_pos = map_offset.1 + y - MAIN_AREA_MARGIN_TOP;
                curses.move_rc(y as i32, x as i32);
                // TODO: Set tile color and char
                let pos = Position::new()
                    .with_x(x_pos as u8)
                    .with_y(y_pos as u8)
                    .with_z(cursor.0.z());
                if let Some(tile) = chunk.tiles.get(pos.position_index()) {
                    let c = char::from(*(tile));
                    curses.print_char(c);
                } else {
                    eprintln!(
                        "Missing tile at location {}, {}, {} (position index {}).",
                        x_pos,
                        y_pos,
                        cursor.0.z(),
                        pos.position_index()
                    );
                }
            }
        }
    } else {
        eprintln!("No chunk data for this chunk!");
    }

    // ---- Map visibility sides ----

    // how much you need to render > the space you have available
    let (edge_bottom, edge_top, edge_left, edge_right) = (
        CHUNK_SIZE_Y as u32 - map_offset.1
            > render.screen_height - MAIN_AREA_MARGIN_TOP - MAIN_AREA_MARGIN_BOTTOM,
        map_offset.1 > 0,
        map_offset.0 > 0,
        CHUNK_SIZE_X as u32 - map_offset.0
            > render.screen_width - MAIN_AREA_MARGIN_LEFT - MAIN_AREA_MARGIN_RIGHT,
    );

    curses.set_color_pair(*COLOR_EDGE);

    // Top Border
    if edge_top {
        for x in MAIN_AREA_MARGIN_LEFT..xmax {
            curses.move_rc(MAIN_AREA_MARGIN_TOP as i32, x as i32);
            curses.print_char('^');
        }
    }

    if edge_left {
        for y in MAIN_AREA_MARGIN_TOP..ymax {
            curses.move_rc(y as i32, MAIN_AREA_MARGIN_LEFT as i32);
            curses.print_char('<');
        }
    }

    if edge_bottom {
        for x in MAIN_AREA_MARGIN_LEFT..xmax {
            curses.move_rc(
                (render.screen_height - MAIN_AREA_MARGIN_BOTTOM - 1) as i32,
                x as i32,
            );
            curses.print_char('v');
        }
    }

    if edge_right {
        for y in MAIN_AREA_MARGIN_TOP..ymax {
            curses.move_rc(
                y as i32,
                (render.screen_width - MAIN_AREA_MARGIN_RIGHT - 1) as i32,
            );
            curses.print_char('>');
        }
    }

    // ---- UI ----

    curses.set_color_pair(*COLOR_TITLE);

    // Debug Info
    curses.move_rc(0, 0);
    curses.print("World of The Fox God V0.1A");

    curses.set_color_pair(*COLOR_DEBUG);

    curses.move_rc(1, 0);
    curses.print(format!(
        "Chunk: {},{} Position: {},{},{}",
        cursor.0.chunk_x(),
        cursor.0.chunk_y(),
        cursor.0.x(),
        cursor.0.y(),
        cursor.0.z(),
    ));

    // Sidebar Test
    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(4, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("Some Things");
    curses.move_rc(5, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("And More");

    // Map Cursor

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(
        (cursor.0.y() as u32 + MAIN_AREA_MARGIN_TOP - map_offset.1) as i32,
        (cursor.0.x() as u32 + MAIN_AREA_MARGIN_LEFT - map_offset.0) as i32,
    );
    curses.print_char(acs::block());

    Ok(())
}
