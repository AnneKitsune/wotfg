use crate::*;

pub fn crafting_input_system(
    item_transition_defs: &ItemTransitionDefinitions<ItemTransitions, Items, Effectors, Stats>,
    input_ev: &mut Vec<InputEvent>,
    selected_craft: &mut SelectedCraft,
) -> SystemResult {
    let max = item_transition_defs.defs.len() as u32;
    let mut out = vec![];
    if let Some(sel) = selected_craft.selected.as_mut() {
        for ev in input_ev.iter() {
            match ev {
                InputEvent::SelectUp => {
                    if *sel > 0 {
                        *sel -= 1;
                    }
                },
                InputEvent::SelectDown => {
                    *sel += 1;
                },
                InputEvent::Accept => {
                    if *sel < max {
                        out.push(*sel);
                    }
                },
                _ => {},
            }
        }
    }
    // Clamp selection to bounds.
    if let Some(sel) = selected_craft.selected {
        if sel >= max {
            if max > 0 {
                selected_craft.selected = Some(max - 1);
            } else {
                selected_craft.selected = None;
            }
        }
    } else {
        if max > 0 {
            selected_craft.selected = Some(0);
        }
    }

    for o in out {
        input_ev.push(InputEvent::AttemptStartCraft(o));
    }
    Ok(())
}
