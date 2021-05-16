use crate::*;

pub fn mine_system(
    players: &Components<Player>,
    positions: &Components<Position>,
    actions: &PlayerActionQueue,
    chunks: &mut Vec<Chunk>,
    inventories: &mut Components<Inventory<Items, (), ItemProperties>>,
) -> SystemResult {
    if let Some(PlayerAction::Mine(direction)) = actions.queue.front() {
        for (player, position, inventory) in join!(&players && &positions && &mut inventories) {
            // TODO check that player id match action source
            // TODO finish
        }
    }
    Ok(())
}
