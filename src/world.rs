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

// 4GB in total, assuming 1 byte per chunk.
pub const WORLD_WIDTH_HEIGHT: u32 = 125;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub tiles: Vec<Tiles>,
    pub collisions: Vec<CollisionMap>,
}

impl Chunk {
    pub fn new_rand(rng: &mut RNG, tile_defs: &TileDefinitions) -> Self {
        let mut tiles = vec![];
        let mut collisions = vec![
            CollisionMap::new(CHUNK_SIZE_X as u32, CHUNK_SIZE_Y as u32);
            CHUNK_SIZE_Z as usize
        ];
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
                    if tile_defs.defs.get(&tile).expect("Tried to generate chunk using tile present in ids but not in tile defs.").solid {
                        collisions
                            .get_mut(z as usize)
                            .unwrap()
                            .set(x as u32, y as u32);
                    }
                }
            }
        }
        Self { tiles, collisions }
    }
    pub fn from_tiles(tiles: Vec<Tiles>, tile_defs: &TileDefinitions) -> Self {
        let mut collisions = vec![
            CollisionMap::new(CHUNK_SIZE_X as u32, CHUNK_SIZE_Y as u32);
            CHUNK_SIZE_Z as usize
        ];
        let idx = 0;
        // TODO accessing arrays in this order might be inneficient.
        for x in 0..CHUNK_SIZE_X as u32 {
            for y in 0..CHUNK_SIZE_Y as u32 {
                for z in 0..CHUNK_SIZE_Z {
                    let tile = tiles.get(idx).expect("Missing tile in chunk that is being loaded!");
                    if tile_defs.defs.get(&tile).expect("Tried to generate chunk using tile present in ids but not in tile defs.").solid {
                        collisions
                            .get_mut(z as usize)
                            .unwrap()
                            .set(x as u32, y as u32);
                    }
                }
            }
        }
        Self { tiles, collisions }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Tiles {
    Air,
    Grass,
    GrassLong,
    Border,
    Bedrock,
    Tree,
    Rock,
    SeliOre,
    GemStoneOre,
    Stone,
}

// TODO move to tile definition once minigene has proper color support.
impl From<Tiles> for ColorPair {
    fn from(t: Tiles) -> Self {
        match t {
            Tiles::Air => ColorPair::new(Color::White, Color::Black),
            Tiles::Grass => ColorPair::new(Color::White, Color::Black),
            Tiles::GrassLong => ColorPair::new(Color::White, Color::Black),
            Tiles::Border => ColorPair::new(Color::Yellow, Color::Black),
            Tiles::Bedrock => ColorPair::new(Color::White, Color::Black),
            Tiles::Tree => ColorPair::new(Color::Yellow, Color::Black),
            Tiles::Rock => ColorPair::new(Color::White, Color::Black),
            Tiles::SeliOre => ColorPair::new(Color::Yellow, Color::Black),
            Tiles::GemStoneOre => ColorPair::new(Color::Green, Color::Black),
            Tiles::Stone => ColorPair::new(Color::White, Color::Black),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TileDefinition {
    pub key: Tiles,
    pub name: String,
    pub harvest_types: Vec<(HarvestType, u32)>,
    pub harvest_time: f32,
    pub drops: Vec<(Items, usize)>,
    pub replace_with: Tiles,
    pub character: char,
    pub solid: bool,
    pub icon: Option<String>,
}

/// The definitions of all known stats.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct TileDefinitions {
    /// The definitions.
    pub defs: HashMap<Tiles, TileDefinition>,
}

impl Default for TileDefinitions {
    fn default() -> Self {
        Self {
            defs: HashMap::default(),
        }
    }
}

impl From<Vec<TileDefinition>> for TileDefinitions {
    fn from(t: Vec<TileDefinition>) -> Self {
        let defs = t
            .into_iter()
            .map(|s| (s.key.clone(), s))
            .collect::<HashMap<_, _>>();
        Self::new(defs)
    }
}
