use crate::*;

pub fn mine_system(
    players: &Components<Player>,
    positions: &Components<Position>,
    actions: &PlayerActionQueue,
    tile_defs: &TileDefinitions,
    items: &mut Components<ItemInstance<Items, ()>>,
    chunks: &mut HashMap<(u32, u32), Chunk>,
) -> SystemResult {
    if let Some(PlayerAction::Mine(direction)) = actions.queue.front() {
        for (player, position) in join!(&players && &positions) {
            // TODO check that player id match action source
            let mut target = position.unwrap().clone();
            target.move_towards(*direction);
            if let Some(chunk) = chunks.get_mut(&(target.chunk_x(), target.chunk_y())) {
                let idx = target.position_index();
                let tile = chunk.tiles.get_mut(idx).expect("Tried to mine missing tile in chunk.");
                let tile_def = tile_defs.defs.get(tile).expect("Tile found in chunk but not in tile defs.");
                let mut can_mine = true;
                for condition in tile_def.harvest_types.iter() {
                    // TODO check condition
                }
                if can_mine {
                    *tile = tile_def.replace_with;
                    for drop in tile_def.drops.iter() {
                        // TODO drop items on the ground
                    }
                }
            }
        }
    }
    Ok(())
}
