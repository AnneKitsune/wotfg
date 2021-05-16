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
    pub collisions: Vec<CollisionMap>,
}

impl Chunk {
    pub fn new_rand(rng: &mut RNG) -> Self {
        let mut tiles = vec![];
        let mut collisions = vec![CollisionMap::new(CHUNK_SIZE_X as u32, CHUNK_SIZE_Y as u32); CHUNK_SIZE_Z as usize];
        for x in 0..CHUNK_SIZE_X {
            for y in 0..CHUNK_SIZE_Y {
                for z in 0..CHUNK_SIZE_Z {
                    let random_number = rng.rng.rand_range(1..1001);
                    let mut tile = Tiles::Air;

                    match z {
                        1..=7 => match random_number {
                            0..=1 => tile = Tiles::SeliOre,
                            2..=3 => tile = Tiles::GemStoneOre,
                            4..=1000 => tile = Tiles::Stone,
                            _ => {}
                        },
                        8 => match random_number {
                            0..=1 => tile = Tiles::Tree,
                            2..=3 => tile = Tiles::Rock,
                            4..=50 => tile = Tiles::GrassLong,
                            51..=1000 => tile = Tiles::Grass,
                            // unreachable
                            _ => {}
                        },
                        9..=15 => tile = Tiles::Air,
                        _ => {}
                    }
                    if z == 0 {
                        tile = Tiles::Bedrock;
                    }
                    if x == 0 || y == 0 || x == CHUNK_SIZE_X - 1 || y == CHUNK_SIZE_Y - 1 {
                        tile = Tiles::Border;
                    }
                    tiles.push(tile);

                    // TODO move to tile definition
                    if match tile {
                        Tiles::Grass | Tiles::GrassLong | Tiles::Bedrock => false,
                        _ => true,
                    } {
                        collisions.get_mut(z as usize).unwrap().set(x as u32, y as u32);
                    }
                }
            }
        }
        Self { tiles, collisions }
    }
}
