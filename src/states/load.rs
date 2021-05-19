use crate::*;

pub struct LoadState;

impl game_engine_core::State<GameData> for LoadState {
    fn on_start(&mut self, data: &mut GameData) {
        let item_defs: Vec<ItemDefinition<Items, (), ItemProperties>> = ron::de::from_str(
            &String::from_utf8(include_bytes!("../../assets/item_defs.ron").to_vec()).unwrap(),
        )
        .expect("Failed to load file: Invalid format.");
        let item_defs = ItemDefinitions::from(item_defs);
        *data.world.get_mut_or_default::<_>() = item_defs;

        // TODO put 5.0 in a const
        data.world
            .get_mut_or_default::<Time>()
            .set_fixed_time(Duration::from_secs_f32(5.0));

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

        let tile_defs: Vec<TileDefinition> = ron::de::from_str(
            &String::from_utf8(include_bytes!("../../assets/tile_defs.ron").to_vec()).unwrap(),
        )
        .expect("Failed to load file: Invalid format.");
        let tile_defs = TileDefinitions::from(tile_defs);

        generate_world(&mut *data.world.get_mut::<_>().unwrap(), &tile_defs);
        panic!("done!");

        *data.world.get_mut_or_default::<_>() = tile_defs;

        data.world.get_mut::<Auth>().unwrap().id = "123".to_string();

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
        data.world.get_mut::<Components<_>>().unwrap().insert(
            player,
            Player::new("123".to_string(), "jojolepro".to_string()),
        );
        data.world
            .get_mut::<Components<_>>()
            .unwrap()
            .insert(player, Rendered::new('P', *COLOR_TITLE, None, 999));

        let inv_result = std::fs::read(format!(
            "{}/worlds/dev/jojolepro_inventory.ron",
            env!("CARGO_MANIFEST_DIR")
        ));
        if let Ok(inv_str) = inv_result {
            let inv: Inventory<Items, (), ()> =
                ron::de::from_bytes(inv_str.as_slice()).expect("Failed to deserialize");
            data.world
                .get_mut::<Components<_>>()
                .unwrap()
                .insert(player, inv);
        } else {
            data.world
                .get_mut::<Components<_>>()
                .unwrap()
                .insert(player, Inventory::<Items, (), ()>::new_dynamic(0, 9999));
        }
    }

    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        StateTransition::Switch(Box::new(InitState))
    }
}
