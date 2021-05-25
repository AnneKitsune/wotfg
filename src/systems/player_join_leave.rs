use crate::*;

pub fn player_join_leave_system(
    positions: &mut Components<Position>,
    rendered: &mut Components<Rendered>,
    players: &mut Components<Player>,
    entities: &mut Entities,
    inventories: &mut Components<Inventory<Items, (), ()>>,
    server_events: &mut Vec<ServerEvents>,
) -> SystemResult {
    let mut joins = vec![];
    for ev in server_events.iter() {
        match ev {
            ServerEvents::PlayerJoin(player) => {
                joins.push(player.clone());
            }
            _ => {}
        }
    }
    for player in joins {
        // TODO read position from world save file
        // create entity
        let position = Position::new()
            .with_x(0)
            .with_y(0)
            .with_z(0)
            .with_chunk_x(0)
            .with_chunk_y(0);
        server_events.push(ServerEvents::PlayerChangedChunk(
            player.clone(),
            position.chunk_x(),
            position.chunk_y(),
        ));
        let entity = entities.create();
        positions.insert(entity, position);
        players.insert(entity, player);
        rendered.insert(entity, Rendered::new('P', *COLOR_TITLE, None, 999));

        // load inventory from save file. be careful here about player names as this does read
        // accesses to the disk.
        let inv_result = std::fs::read(format!(
            "{}/worlds/dev/jojolepro_inventory.ron",
            env!("CARGO_MANIFEST_DIR")
        ));
        if let Ok(inv_str) = inv_result {
            let inv: Inventory<Items, (), ()> =
                ron::de::from_bytes(inv_str.as_slice()).expect("Failed to deserialize");
            inventories.insert(entity, inv);
        } else {
            inventories.insert(entity, Inventory::<Items, (), ()>::new_dynamic(0, 9999));
        }
    }
    Ok(())
}
