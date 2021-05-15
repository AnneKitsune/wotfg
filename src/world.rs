use crate::*;
// originally, values were 40,40,10
// if we use values that can be divided by a power of two, its easier to store position as a single
// value.
pub const CHUNK_SIZE_X: u8 = 128;
pub const CHUNK_SIZE_Y: u8 = 128;
pub const CHUNK_SIZE_Z: u8 = 16;
// sqrt(18446744073709551615 / 128 / 128 / 16)
// or also, 2^23.
pub const CHUNK_COUNT_SQRT: u32 = 8388608;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub tiles: Vec<Tiles>,
    // TODO
    //pub collisions: CollisionMap,
}

impl Chunk {
    pub fn new_rand() -> Self {
        let mut tiles = vec![];
        for x in 0..CHUNK_SIZE_X {
            for y in 0..CHUNK_SIZE_Y {
                for z in 0..CHUNK_SIZE_Z {
                    let mut tile = match (x + y) % 20 {
                        0..=15 => Tiles::Grass,
                        16..=18 => Tiles::Tree,
                        19..=20 => Tiles::Air,
                        // unreachable
                        _ => Tiles::Air,
                    };
                    if x == 0 || y == 0 || x == CHUNK_SIZE_X - 1 || y == CHUNK_SIZE_Y - 1 {
                        tile = Tiles::Border;
                    }
                    if z == 0 {
                        tile = Tiles::Bedrock;
                    }
                    tiles.push(tile);
                }
            }
        }
        Self { tiles }
    }
}
