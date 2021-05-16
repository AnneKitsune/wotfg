use crate::*;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
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
    GrassFiber,
    Log,
    Rock,
    SeliOre,
    GemStone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
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

// TODO use HarvestType in tools along with DamageType
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum HarvestType {
    Mining,
    PlantGathering,
    Butchering,
    Shoveling,
    Chopping,
    Exploding,
}

// Some discrete stats like Magical Crafting V are actually passive skills unlocked
// using the magical_crafting_xp stat.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Skills {
    AfterlifeEfficiency,
    MythicalComprehension1,
    MythicalCrafting1,
}

// Switch to using effectors directly?
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum DamageType {
    Physical,
    Magical,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Effectors {}

#[derive(Hash, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ItemTransitions {
    CraftUnobtainiumPlatesChestpieceTier8,
}
