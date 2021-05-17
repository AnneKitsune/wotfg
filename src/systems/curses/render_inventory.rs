use crate::*;

// 3 columns
// left is selected item description
// middle is inventory
// right is containers and equipped items
pub fn curses_render_inventory_system(
    controlled: &Components<Player>,
    inventories: &Components<Inventory<Items, (), ()>>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    render: &RenderInfo,
    auth: &Auth,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    // TODO probably shouldn't be clearing from here. should have a dedicated system instead.
    curses.set_color_pair(*COLOR_NORMAL);
    for y in (MAIN_AREA_MARGIN_TOP as i32)
        ..(render.screen_height as i32 - MAIN_AREA_MARGIN_BOTTOM as i32)
    {
        for x in (MAIN_AREA_MARGIN_LEFT as i32)
            ..(render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32)
        {
            curses.move_rc(y, x);
            curses.print_char(' ');
        }
    }

    let start = MAIN_AREA_MARGIN_LEFT as i32;
    let third =
        (render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32 - MAIN_AREA_MARGIN_LEFT as i32)
            / 3
            + MAIN_AREA_MARGIN_LEFT as i32;
    let center =
        (render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32 - MAIN_AREA_MARGIN_LEFT as i32)
            / 2
            + MAIN_AREA_MARGIN_LEFT as i32;
    let third2 =
        (render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32 - MAIN_AREA_MARGIN_LEFT as i32)
            / 3
            * 2
            + MAIN_AREA_MARGIN_LEFT as i32;
    let end = render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32;

    curses.set_color_pair(*COLOR_NORMAL);
    let s = "===== Inventory =====";
    let half = s.len() as i32 / 2;
    curses.move_rc(MAIN_AREA_MARGIN_TOP as i32, center - half);
    curses.print(s);

    let mut y = MAIN_AREA_MARGIN_TOP as i32 + 2;
    for (player, inv) in join!(&controlled && &inventories) {
        if player.unwrap().id == auth.id {
            for item in inv.as_ref().unwrap().content.iter() {
                if item.is_some() {
                    let def = item_defs
                        .defs
                        .get(&item.as_ref().unwrap().key)
                        .unwrap_or_else(|| {
                            panic!(
                                "Failed to find item def for item key {:?}",
                                item.as_ref().unwrap().key
                            )
                        });
                    curses.move_rc(y, third + 1);
                    curses.set_color_pair(def.user_data.rarity.into());
                    curses.print(format!("{} x{}", def.name, item.as_ref().unwrap().quantity));
                    curses.move_rc(y + 1, third + 1);
                    curses.set_color_pair(*COLOR_NORMAL);
                    curses.print(format!(">{}", def.description));
                    y += 2;
                }
            }
        }
    }
    Ok(())
}
