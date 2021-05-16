use crate::*;

pub fn curses_render_crafting_system(
    controlled: &Components<Player>,
    inventories: &Components<Inventory<Items, (), ItemProperties>>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    stat_defs: &StatDefinitions<Stats>,
    transitions: &ItemTransitionDefinitions<ItemTransitions, Items, Effectors, Stats>,
    render: &RenderInfo,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    //roman::to(level).unwrap();
    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(16, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("=== Craft ===");
    let mut y = 17;
    for trans in transitions.defs.values() {
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
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            let rarest_color = ColorPair::from(
                output_defs
                    .iter()
                    .map(|d| d.user_data.rarity)
                    .max()
                    .expect("Failed to order rarities in crafting recipe."),
            );
            curses.set_color_pair(rarest_color);
            curses.print(format!("{}", trans.name));
            y += 1;
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            curses.set_color_pair(*COLOR_NORMAL);
            curses.print(format!("===================="));
            y += 1;

            // print time to craft
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            curses.print(format!(
                "Time to craft: {} Seconds",
                trans.time_to_complete as u64
            ));
            y += 1;

            // print materials
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            curses.print(format!("Materials:"));
            y += 1;

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
                curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                curses.set_color_pair(ColorPair::from(idef.user_data.rarity));
                curses.print(format!("- {}x {}", ik.1, idef.name));
                y += 1;
            }

            // print minimum skill requirements
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            curses.set_color_pair(*COLOR_NORMAL);
            curses.print(format!("Minimum Skill Requirements:"));
            y += 1;

            for cond in trans.stat_conditions.iter() {
                let stat_def = stat_defs
                    .defs
                    .get(&cond.stat_key)
                    .expect("Item Transition references stat not present in stat definitions.");
                if let StatConditionType::MinValue(min) = cond.condition {
                    curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                    let roman_level =
                        roman::to(min as i32).expect("Failed to convert required level to roman.");
                    curses.print(format!("- {} {}", stat_def.name, roman_level));
                    y += 1;
                }
            }

            // print tools
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            curses.set_color_pair(*COLOR_NORMAL);
            curses.print(format!("Tools:"));
            y += 1;

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
                curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                curses.set_color_pair(ColorPair::from(idef.user_data.rarity));
                curses.print(format!("- {}", idef.name));
                y += 1;
            }

            // print success chance TODO
            curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
            curses.set_color_pair(*COLOR_NORMAL);
            curses.print(format!("Success Chance with Current Skills: 85"));
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
