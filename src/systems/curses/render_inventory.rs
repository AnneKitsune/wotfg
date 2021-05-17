use crate::*;

pub fn curses_render_inventory_system(
    controlled: &Components<Player>,
    inventories: &Components<Inventory<Items, (), ()>>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    render: &RenderInfo,
    auth: &Auth,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(6, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("=== Inventory ===");
    let mut y = 7;
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
                    curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                    curses.print(format!("{} x{}", def.name, item.as_ref().unwrap().quantity));
                    curses.move_rc(y + 1, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                    curses.print(format!(">{}", def.description));
                    y += 2;
                }
            }
        }
    }
    Ok(())
}
