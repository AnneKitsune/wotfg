use crate::*;

pub fn mine_system(
    players: &Components<Player>,
    positions: &Components<Position>,
    actions: &Vec<PlayerAction>,
    chunks: &mut Vec<Chunk>,
    inventories: &mut Components<Inventory<Items, (), ItemProperties>>,
) -> SystemResult {
    for a in actions {
        if let PlayerAction::Mine(direction) = a {
            for (player, position, inventory) in join!(&players && &positions && &mut inventories) {
                // TODO check that player id match action source
                // TODO finish
            }
        }
    }
    Ok(())
}
