use crate::*;

pub struct LoadState;

impl game_engine_core::State<GameData> for LoadState {
    fn on_start(&mut self, data: &mut GameData) {
        data.world
            .get_mut::<HashMap<(u32, u32), Chunk>>()
            .unwrap()
            .insert((0, 0), Chunk::new_rand(&mut data.world.get_mut::<_>().unwrap()));
        data.world
            .get_mut::<HashMap<(u32, u32), Chunk>>()
            .unwrap()
            .insert((0, 1), Chunk::new_rand(&mut data.world.get_mut::<_>().unwrap()));
        data.world
            .get_mut::<HashMap<(u32, u32), Chunk>>()
            .unwrap()
            .insert((1, 0), Chunk::new_rand(&mut data.world.get_mut::<_>().unwrap()));
        data.world
            .get_mut::<HashMap<(u32, u32), Chunk>>()
            .unwrap()
            .insert((1, 1), Chunk::new_rand(&mut data.world.get_mut::<_>().unwrap()));

        let mut inv = Inventory::<Items, (), ItemProperties>::new_dynamic(0, 9999);
        inv.insert(ItemInstance::new(Items::TestItemA, 1))
            .expect("Failed to insert init item into inventory.");
        inv.get_mut(0).unwrap().quantity = 2;
        inv.insert(ItemInstance::new(Items::RustyDagger, 1))
            .expect("Failed to insert init item into inventory.");
        inv.insert(ItemInstance::new(Items::MagicalGauntlet, 1))
            .expect("Failed to insert init item into inventory.");

        let item_defs: Vec<ItemDefinition<Items, (), ItemProperties>> = ron::de::from_str(
            &String::from_utf8(include_bytes!("../../assets/item_defs.ron").to_vec()).unwrap(),
        )
        .expect("Failed to load file: Invalid format.");
        let item_defs = ItemDefinitions::from(item_defs);

        *data.world.get_mut_or_default::<_>() = item_defs;

        let stat_defs: Vec<StatDefinition<Stats>> = ron::de::from_str(
            &String::from_utf8(include_bytes!("../../assets/stat_defs.ron").to_vec()).unwrap(),
        )
        .expect("Failed to load file: Invalid format.");
        let stat_defs = StatDefinitions::from(stat_defs);
        *data.world.get_mut_or_default::<_>() = stat_defs;

        let transitions_defs: Vec<
            ItemTransitionDefinition<ItemTransitions, Items, Effectors, Stats>,
        > = ron::de::from_str(
            &String::from_utf8(include_bytes!("../../assets/item_transition_defs.ron").to_vec())
                .unwrap(),
        )
        .expect("Failed to load file: Invalid format.");
        let transitions_defs = ItemTransitionDefinitions::from(transitions_defs);
        *data.world.get_mut_or_default::<_>() = transitions_defs;

        let player = data.world.get_mut::<Entities>().unwrap().create();
        data.world.get_mut::<Components<_>>().unwrap().insert(
            player,
            Position::new()
                .with_x(0)
                .with_y(0)
                .with_z(0)
                .with_chunk_x(0)
                .with_chunk_y(0),
        );
        data.world
            .get_mut::<Components<_>>()
            .unwrap()
            .insert(player, Controlled);
        data.world
            .get_mut::<Components<_>>()
            .unwrap()
            .insert(player, Rendered::new('P', *COLOR_TITLE, None));
        data.world
            .get_mut::<Components<_>>()
            .unwrap()
            .insert(player, inv);
    }

    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        StateTransition::Switch(Box::new(InitState))
    }
}
