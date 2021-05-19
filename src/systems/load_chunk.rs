use crate::*;
use std::collections::HashSet;

pub fn load_chunks_system(players: &Components<Player>, positions: &Components<Position>, tile_defs: &TileDefinitions, events: &mut Vec<ServerEvents>, chunks: &mut HashMap<(u32, u32), Chunk>) -> SystemResult {
    let current = chunks.keys().map(|k| *k).collect::<HashSet<_>>();
    let mut next = HashSet::<(u32, u32)>::default();
    for (position, _) in join!(&positions && &players) {
        next.insert((position.unwrap().chunk_x(), position.unwrap().chunk_y()));
    }
    let load = next.difference(&current);
    let unload = current.difference(&next);

    for l in load {
        // load from disk
        let chunk = Chunk::from_disk(l.0, l.1, "dev".to_string(), tile_defs);
        chunks.insert(*l, chunk);
        events.push(ServerEvents::ChunkLoaded(l.0, l.1));
    }
    for l in unload {
        // save to disk
        chunks.remove(l).unwrap().to_disk(l.0, l.1, "dev".to_string());
        events.push(ServerEvents::ChunkUnloaded(l.0, l.1));
    }
    Ok(())
}
