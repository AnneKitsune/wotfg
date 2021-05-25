use crate::*;

// left panel = all crafting recipes
// right panel = details
pub fn curses_render_crafting_system(
    controlled: &Components<Player>,
    inventories: &Components<Inventory<Items, (), ()>>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    stat_defs: &StatDefinitions<Stats>,
    transitions: &ItemTransitionDefinitions<ItemTransitions, Items, Effectors, Stats>,
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
    let center =
        (render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32 - MAIN_AREA_MARGIN_LEFT as i32)
            / 2
            + MAIN_AREA_MARGIN_LEFT as i32;
    let end = render.screen_width as i32 - MAIN_AREA_MARGIN_RIGHT as i32;

    curses.set_color_pair(*COLOR_NORMAL);
    let s = "===== Crafting =====";
    let half = s.len() as i32 / 2;
    curses.move_rc(MAIN_AREA_MARGIN_TOP as i32, center - half);
    curses.print(s);

    // TODO replace by resource
    let selected_recipe = 0;

    //roman::to(level).unwrap();
    let mut y_list = (MAIN_AREA_MARGIN_TOP + 1) as i32;
    let mut y_description = (MAIN_AREA_MARGIN_TOP + 1) as i32;
    for (number, trans) in transitions.defs.values().enumerate() {
        // crafting recipes
        if trans.auto_trigger == false {
            let output_defs = trans
                .output_items
                .iter()
                .map(|k| {
                    item_defs
                        .defs
                        .get(&k.0)
                        .expect("Item Transition references item not present in item definitions.")
                })
                .collect::<Vec<_>>();
            if output_defs.is_empty() {
                continue;
            }

            // print craft title
            let rarest_color = ColorPair::from(
                output_defs
                    .iter()
                    .map(|d| d.user_data.rarity)
                    .max()
                    .expect("Failed to order rarities in crafting recipe."),
            );
            curses.move_rc(y_list, start + 1);
            curses.set_color_pair(rarest_color);
            curses.move_rc(y_list, start);
            if number == selected_recipe {
                curses.print(">");
                curses.move_rc(y_description, start);
                curses.set_color_pair(rarest_color);
                curses.print(format!("{}", trans.name));

                y_description += 1;
                curses.move_rc(y_description, center);
                curses.set_color_pair(*COLOR_NORMAL);
                curses.print(format!("===================="));
                y_description += 1;

                // print time to craft
                curses.move_rc(y_description, center);
                curses.print(format!(
                    "Time to craft: {} Seconds",
                    trans.time_to_complete as u64
                ));
                y_description += 1;

                // print materials
                curses.move_rc(y_description, center);
                curses.print(format!("Materials:"));
                y_description += 1;

                for ik in trans.input_items.iter().filter(|(_, _, mode)| {
                    if let UseMode::Consume = *mode {
                        true
                    } else {
                        false
                    }
                }) {
                    let idef = item_defs
                        .defs
                        .get(&ik.0)
                        .expect("Item Transition references item not present in item definitions.");
                    curses.move_rc(y_description, center);
                    curses.set_color_pair(ColorPair::from(idef.user_data.rarity));
                    curses.print(format!("- {}x {}", ik.1, idef.name));
                    y_description += 1;
                }

                // print minimum skill requirements
                curses.move_rc(y_description, center);
                curses.set_color_pair(*COLOR_NORMAL);
                curses.print(format!("Minimum Skill Requirements:"));
                y_description += 1;

                for cond in trans.stat_conditions.iter() {
                    let stat_def = stat_defs
                        .defs
                        .get(&cond.stat_key)
                        .expect("Item Transition references stat not present in stat definitions.");
                    if let StatConditionType::MinValue(min) = cond.condition {
                        curses.move_rc(y_description, center);
                        let roman_level = roman::to(min as i32)
                            .expect("Failed to convert required level to roman.");
                        curses.print(format!("- {} {}", stat_def.name, roman_level));
                        y_description += 1;
                    }
                }

                // print tools
                curses.move_rc(y_description, center);
                curses.set_color_pair(*COLOR_NORMAL);
                curses.print(format!("Tools:"));
                y_description += 1;

                for ik in trans.input_items.iter().filter(|(_, _, mode)| {
                    if let UseMode::Consume = *mode {
                        false
                    } else {
                        true
                    }
                }) {
                    let idef = item_defs
                        .defs
                        .get(&ik.0)
                        .expect("Item Transition references item not present in item definitions.");
                    curses.move_rc(y_description, center);
                    curses.set_color_pair(ColorPair::from(idef.user_data.rarity));
                    curses.print(format!("- {}", idef.name));
                    y_description += 1;
                }

                // print success chance TODO
                curses.move_rc(y_description, center);
                curses.set_color_pair(*COLOR_NORMAL);
                curses.print(format!("Success Chance with Current Skills: 85"));
            } else {
                curses.print(" ");
            }
        }
    }
    /*for (_, inv) in join!(&controlled && &inventories) {
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
            }
        }
    }*/
    Ok(())
}
