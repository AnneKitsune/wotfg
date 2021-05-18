use crate::*;

pub fn quick_select_system(
    controlled: &Components<Player>,
    items: &Components<ItemInstance<Items, ()>>,
    positions: &Components<Position>,
    auth: &Auth,
    input_ev: &Vec<InputEvent>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    selected_item: &mut QuickItemSelect,
    inventories: &mut Components<Inventory<Items, (), ()>>,
    entities: &mut Entities,
) -> SystemResult {
    for (player, player_position, mut inventory) in
        join!(&controlled && &positions && &mut inventories)
    {
        if player.unwrap().id == auth.id {
            let mut close = vec![];
            for (entity, item, item_position) in join!(&entities && &items && &positions) {
                // TODO move this check in a reusable function
                if player_position.unwrap().distance(item_position.unwrap()) <= PICKUP_DISTANCE
                    && player_position.unwrap().z() == item_position.unwrap().z()
                {
                    close.push((entity.unwrap(), item.unwrap().clone()));
                }
            }

            if let Some(sel) = selected_item.selected.as_mut() {
                for ev in input_ev {
                    match ev {
                        InputEvent::SelectUp => {
                            if *sel > 0 {
                                *sel -= 1;
                            }
                        }
                        InputEvent::SelectDown => {
                            *sel += 1;
                        }
                        InputEvent::Accept => {
                            // TODO move this to player action pick up item, once we have entity
                            // network identifiers.
                            if (*sel as usize) < close.len() {
                                let (entity, item) = close.remove(*sel as usize);
                                if let Err(e) = inventory.as_mut().unwrap().insert(item, item_defs)
                                {
                                    // TODO better error handling
                                    eprintln!("Failed to insert item in inventory.");
                                } else {
                                    entities.kill(entity);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            // Clamp selection to bounds.
            if let Some(sel) = selected_item.selected {
                if sel >= close.len() as u32 {
                    if close.len() > 0 {
                        selected_item.selected = Some(close.len() as u32 - 1);
                    } else {
                        selected_item.selected = None;
                    }
                }
            } else {
                if close.len() > 0 {
                    selected_item.selected = Some(0);
                }
            }
        }
    }
    Ok(())
}
