use crate::*;

// Server-side crafting system.
pub fn crafting_system(
    player_actions: &PlayerActionQueue,
    players: &Components<Player>,
    statsets: &Components<StatSet<Stats>>,
    stat_defs: &StatDefinitions<Stats>,
    transitions: &ItemTransitionDefinitions<ItemTransitions, Items, Effectors, Stats>,
    inventories: &mut Components<Inventory<Items, (), ()>>,
) -> SystemResult {
    if let Some(a) = player_actions.queue.front() {
        PlayerAction::StartCraft(key) => {
            for (player, inventory, statset) in join!(&players && &mut inventories && &statsets) {
                let inv = inventory.as_mut().unwrap();
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
                        // start the craft
                    }
                }
            }
        },
        _ => {};
    }
    Ok(())
}
