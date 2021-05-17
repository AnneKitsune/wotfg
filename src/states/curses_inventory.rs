use crate::*;

pub struct InventoryState;

impl game_engine_core::State<GameData> for InventoryState {
    fn on_start(&mut self, data: &mut GameData) {
    }

    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        data.logic_dispatcher
            .run_seq(&mut data.world)
            .expect("Failed to run systems.");
        data.world.maintain();
        data.render_inventory_dispatcher
            .run_seq(&mut data.world)
            .expect("Failed to run systems.");
        data.world.maintain();

        StateTransition::None
    }
}
