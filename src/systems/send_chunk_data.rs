use crate::*;

// server system
// TODO network
pub fn send_chunk_data_system(
    events: &Vec<ServerEvents>,
    chunks: &HashMap<(u32, u32), Chunk>,
    tmp: &mut Option<Chunk>,
) -> SystemResult {
    for ev in events {
        match ev {
            ServerEvents::PlayerChangedChunk(_, x, y) => {
                *tmp = chunks.get(&(*x, *y)).cloned();
            }
            _ => {}
        }
    }
    Ok(())
}
