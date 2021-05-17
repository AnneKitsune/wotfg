use crate::*;

pub struct GameData {
    pub render_dispatcher: Dispatcher,
    pub render_inventory_dispatcher: Dispatcher,
    pub render_crafting_dispatcher: Dispatcher,
    pub logic_dispatcher: Dispatcher,
    pub world: World,
}
