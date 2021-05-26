// validate that we can craft before sending the request to the server. this is
// to prevent creating PlayerActions that are not needed, which would look weird
// on the ui

use crate::*;

pub fn client_start_craft_system(input_evs: &Vec<InputEvent>, players: &Components<Player>, 
    auth: &Auth,
    statsets: &Components<StatSet<Stats>>,
    stat_defs: &StatDefinitions<Stats>,
    transitions: &ItemTransitionDefinitions<ItemTransitions, Items, Effectors, Stats>,
    inventories: &Components<Inventory<Items, (), ()>>,
    player_actions: &mut PlayerActionQueue) -> SystemResult {
    for ev in input_evs {
        match ev {
            InputEvent::AttemptStartCraft(id) => {
                for (player, inventory, statset) in join!(&players && &inventories && &statsets) {
                    if player.unwrap().id == auth.id {
                        let inv = inventory.unwrap();
                        let statset = statset.unwrap();
                        if let Some(trans) = transitions.defs.values().filter(|t| !t.auto_trigger).nth(*id as usize) {
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

                            // if all good, push player action
                            if ok {
                                player_actions.queue.push_back(PlayerAction::StartCraft(trans.key));
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
