use crate::*;

pub struct InventoryState;

impl game_engine_core::State<GameData> for InventoryState {
    fn on_start(&mut self, data: &mut GameData) {
        let invs = data
            .world
            .get::<Components<Inventory<Items, (), ()>>>()
            .unwrap();
        let inv = invs.iter().next().unwrap();
        let inv_str = ron::ser::to_string(&*inv).expect("Failed to serialize");
        std::fs::write(
            format!(
                "{}/worlds/dev/jojolepro_inventory.ron",
                env!("CARGO_MANIFEST_DIR")
            ),
            inv_str,
        )
        .expect("Failed to write inventory to file.");
    }
    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        while data.world.get_mut::<Time>().unwrap().step_fixed_update() {
            data.logic_dispatcher
                .run_seq(&mut data.world)
                .expect("Failed to run systems.");
            data.world.maintain();
        }
        data.render_inventory_dispatcher
            .run_seq(&mut data.world)
            .expect("Failed to run systems.");
        let mut trans = StateTransition::None;
        for ev in data.world.get::<Vec<InputEvent>>().unwrap().iter() {
            match ev {
                InputEvent::Crafting => trans = StateTransition::Push(Box::new(CraftingState)),
                InputEvent::Cancel => trans = StateTransition::Pop,
                _ => {}
            }
        }
        clear_events.system().run(&mut data.world).unwrap();
        data.world.maintain();

        trans
    }
}
