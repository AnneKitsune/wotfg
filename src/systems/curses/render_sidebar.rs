use crate::*;

pub fn curses_render_sidebar_system(
    players: &Components<Player>,
    inventories: &Components<Inventory<Items, (), ()>>,
    stat_defs: &StatDefinitions<Stats>,
    stats: &Components<StatSet<Stats>>,
    render: &RenderInfo,
    auth: &Auth,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    let left = render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32;
    let right = render.screen_width as i32;
    let center = left + (right - left) / 2;

    // clear anything that could have been written on top of the sidebar
    curses.set_color_pair(*COLOR_NORMAL);
    for y in 0..render.screen_height as i32
    {
        for x in left..right
        {
            curses.move_rc(y, x);
            curses.print_char(' ');
        }
    }

    curses.move_rc(0, center - 3);
    curses.print("Status");
    for x in left..right {
        curses.move_rc(0, x);
        curses.print_char('=');
    }

    let mut y = 2;

    for (player, statset) in join!(&players && &stats) {
        let statset = statset.unwrap();
        if auth.id == player.unwrap().id {
            let name = player.unwrap().username.clone();
            let health = statset.stats.get(&Stats::Health).unwrap().value;
            let mana = statset.stats.get(&Stats::Mana).unwrap().value;

            curses.move_rc(y, left);
            curses.print(format!("Name: {}", name));
            y += 1;

            curses.move_rc(y, left);
            curses.print(format!("Health: {}", health));
            y += 1;

            curses.move_rc(y, left);
            curses.print(format!("Mana: {}", mana));
            y += 1;
        }
    }

    Ok(())
}
