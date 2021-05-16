use crate::*;

pub fn mine_system(
    players: &Components<Player>,
    actions: &PlayerActionQueue,
    tile_defs: &TileDefinitions,
    item_defs: &ItemDefinitions<Items, (), ItemProperties>,
    positions: &mut Components<Position>,
    items: &mut Components<ItemInstance<Items, ()>>,
    renderables: &mut Components<Rendered>,
    entities: &mut Entities,
    chunks: &mut HashMap<(u32, u32), Chunk>,
) -> SystemResult {
    if let Some(PlayerAction::Mine(direction)) = actions.queue.front() {
        let mut created_items_positions = vec![];
        for (player, position) in join!(&players && &positions) {
            // TODO check that player id match action source
            let mut target = position.unwrap().clone();
            target.move_towards(*direction);
            if let Some(chunk) = chunks.get_mut(&(target.chunk_x(), target.chunk_y())) {
                let idx = target.position_index();
                let tile = chunk
                    .tiles
                    .get_mut(idx)
                    .expect("Tried to mine missing tile in chunk.");
                let tile_def = tile_defs
                    .defs
                    .get(tile)
                    .expect("Tile found in chunk but not in tile defs.");

                let mut can_mine = true;
                if tile_def.harvest_time <= 0.0 {
                    can_mine = false;
                }
                if can_mine {
                    for condition in tile_def.harvest_types.iter() {
                        // TODO check condition
                    }
                }
                if can_mine {
                    *tile = tile_def.replace_with;
                    let replaced_def = tile_defs
                        .defs
                        .get(tile)
                        .expect("Tile found in chunk but not in tile defs.");
                    if replaced_def.solid {
                        chunk.collisions.get_mut(target.z() as usize).unwrap().set(target.x() as u32, target.y() as u32);
                    } else {
                        chunk.collisions.get_mut(target.z() as usize).unwrap().unset(target.x() as u32, target.y() as u32);
                    }

                    // Drop items created by mining this.
                    for drop in tile_def.drops.iter() {
                        let drop_def = item_defs.defs.get(&drop.0).expect("Dropped item is not in item defs.");
                        let entity = entities.create();
                        let item = ItemInstance::new(drop.0, drop.1 as usize);
                        created_items_positions.push((entity, target.clone()));
                        items.insert(entity, item);
                        renderables.insert(entity, Rendered::new( ',', drop_def.user_data.rarity.into(), None, 0));
                        // TODO drop items on the ground
                    }
                }
            }
        }
        // little hack to get around borrowing issue if using this inside of the main loop.
        for (entity, target) in created_items_positions {
            positions.insert(entity, target);
        }
    }
    Ok(())
}
