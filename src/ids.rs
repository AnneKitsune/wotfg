use crate::*;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum Items {
    TestItemA,
    TestItemB,
    TestItemC,
    RustyDagger,
    MagicalGauntlet,
    UnobtainiumPlatesChestpieceTier8,
    UnobtainiumPlate,
    DemonicGlue,
    DraconicEnergyCore,
    SoulOfTheTrueMage,
    Welder,
    Forge,
    MagicOrbTier8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum Stats {
    Health,
    Mana,
    AttackSpeedMultiplier,
    Temperature,
    MagicalCraftingXp,
    WeavingXp,
    MovementSpeedMultiplier,
    AfterlifeEssence,
    AfterlifeDrain,
    LifeLength,
    MagicHandling,
    MetalForging,
    Gluing,
    MysticalComprehension,
    MysticalCrafting,
}

// Some discrete stats like Magical Crafting V are actually passive skills unlocked
// using the magical_crafting_xp stat.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize)]
pub enum Skills {
    AfterlifeEfficiency,
    MythicalComprehension1,
    MythicalCrafting1,
}

// Switch to using effectors directly?
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum DamageType {
    Physical,
    Magical,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize)]
pub enum Rarity {
    Common,
    Rare,
    VeryRare,
    Epic,
    Mythic,
    Legendary,
    Unobtainable,
    Unique,
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Common
    }
}

impl From<Rarity> for ColorPair {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::Common => ColorPair::new(Color::White, Color::Black),
            Rarity::Rare => ColorPair::new(Color::Cyan, Color::Black),
            Rarity::VeryRare => ColorPair::new(Color::Magenta, Color::Black),
            Rarity::Epic => ColorPair::new(Color::Red, Color::Black),
            Rarity::Mythic => ColorPair::new(Color::Black, Color::White),
            Rarity::Legendary => ColorPair::new(Color::Blue, Color::White),
            Rarity::Unobtainable => ColorPair::new(Color::Cyan, Color::Magenta),
            Rarity::Unique => ColorPair::new(Color::Black, Color::Red),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum Effectors {}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum ItemTransitions {
    CraftUnobtainiumPlatesChestpieceTier8,
}

#[derive(Clone, Copy, Debug)]
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

// TODO do that, but for a tile that has bg and fg color, and a tile texture/animation.
impl From<Tiles> for char {
    fn from(t: Tiles) -> Self {
        match t {
            Tiles::Air => ' ',
            Tiles::Grass => '.',
            Tiles::GrassLong => ',',
            Tiles::Border => 'b',
            Tiles::Bedrock => 'B',
            Tiles::Tree => 'T',
            Tiles::Rock => 'o',
            Tiles::SeliOre => '-',
            Tiles::GemStoneOre => '^',
            Tiles::Stone => '0',
        }
    }
}
