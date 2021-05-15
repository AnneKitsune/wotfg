use crate::*;

pub fn mine_system(
    controlled: &Components<Controlled>,
    positions: &Components<Position>,
    chunks: &Vec<Chunk>,
    inputs: &Vec<InputEvent>,
    inventories: &mut Components<Inventory<Items, (), ItemProperties>>,
) -> SystemResult {
    Ok(())
}

