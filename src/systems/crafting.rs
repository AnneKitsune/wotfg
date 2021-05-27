use crate::*;

// Server-side crafting system.
pub fn crafting_system(
    player_actions: &PlayerActionQueue,
    players: &Components<Player>,
    statsets: &Components<StatSet<Stats>>,
    stat_defs: &StatDefinitions<Stats>,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    transitions: &ItemTransitionDefinitions<ItemTransitions, Items, Effectors, Stats>,
    inventories: &mut Components<Inventory<Items, (), ()>>,
) -> SystemResult {
    if let Some(a) = player_actions.queue.front() {
        match a {
            PlayerAction::StartCraft(key) => {
                for (player, mut inventory, statset) in join!(&players && &mut inventories && &statsets) {
                    let mut inv = inventory.as_mut().unwrap();
                    let statset = statset.unwrap();
                    if let Some(trans) = transitions .defs.get(&key) {
                        // TODO move to game_features
                        let mut ok = true;
                        // check item conditions
                        for i in trans.input_items.iter() {
                            if !inv.has_quantity(&i.0, i.1) {
                                ok = false;
                                break;
                            }
                        }

                        // check stat conditions
                        if ok {
                            for sc in trans.stat_conditions.iter() {
                                if !sc.check(statset, stat_defs) {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                        if ok {
                            // consume resources
                            for i in trans.input_items.iter() {
                                if i.2 == UseMode::Consume {
                                    inv.delete_key(&i.0, i.1).unwrap();
                                }
                            }
                            // start the craft
                            // TODO just start the craft but don't complete it.
                            for i in trans.output_items.iter() {
                                inv.insert(ItemInstance::new(i.0, i.1), item_defs);
                            }
                        }
                    }
                }
            },
            _ => {},
        }
    }
    Ok(())
}
