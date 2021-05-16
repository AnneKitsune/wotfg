//use pathfinding::directed::astar;
use easycurses::constants::acs;
use easycurses::*;
use lazy_static::lazy_static;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Duration;

use derive_new::*;
use game_engine_core::*;
use minigene::*;
use modular_bitfield::prelude::*;

use serde::Deserialize;

mod ids;
mod input;
mod states;
mod systems;
mod world;

pub use self::ids::*;
pub use self::input::*;
pub use self::states::*;
pub use self::systems::*;
pub use self::world::*;

const MAIN_AREA_MARGIN_LEFT: u32 = 0;
const MAIN_AREA_MARGIN_TOP: u32 = 4;
const MAIN_AREA_MARGIN_RIGHT: u32 = 40;
const MAIN_AREA_MARGIN_BOTTOM: u32 = 0;

#[derive(Default, Clone, Debug, Deserialize)]
pub struct ItemProperties {
    pub rarity: Rarity,
    pub damages: Vec<(DamageType, f32)>,
    pub crit_chance: f32,
    pub mining_level: u32,
    pub chopping_level: u32,
    pub attack_speed: f32,
    pub mining_speed: f32,
    pub attack_range: f32,
    pub can_range: bool,
}

#[bitfield]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Position {
    chunk_x: B23, // 8388608
    chunk_y: B23, // 8388608
    x: B7,        // 128
    y: B7,        // 128
    z: B4,        // 16
}

/// An input which requires a second keypress and can be cancelled.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HangedInput {
    /// Requires a direction
    Mine,
    /// Requires a key name
    MacroRecord,
    /// Requires a key name
    MacroReplay,
}

/// Indicates that this entity is directly controlled by the input events.
/// Shouldn't be used once the network is implemented.
#[derive(Clone, Copy, Default, Debug)]
pub struct Controlled;

/// Indicates that this entity should be rendered.
/// The entity must also have a Position component attached if it is inside of the world.
#[derive(new, Clone, Default, Debug)]
pub struct Rendered {
    pub render_char: char,
    // TODO switch to minigene's exported color
    pub color: ColorPair,
    pub texture_path: Option<String>,
}

impl Position {
    /// Returns the position inside the chunk as a single number
    pub fn position_index(&self) -> usize {
        ((self.x() as usize) << 11) | ((self.y() as usize) << 4) | (self.z() as usize)
    }

    // TODO add collision map handling
    pub fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::West => {
                if self.x() == 0 {
                    // change chunk
                    if self.chunk_x() > 0 {
                        self.set_chunk_x(self.chunk_x() - 1);
                        self.set_x(CHUNK_SIZE_X - 1);
                    }
                } else {
                    self.set_x(self.x() - 1);
                }
            }
            Direction::East => {
                if self.x() >= CHUNK_SIZE_X - 1 {
                    // change chunk
                    if self.chunk_x() < CHUNK_COUNT_SQRT - 1 {
                        self.set_chunk_x(self.chunk_x() + 1);
                        self.set_x(0);
                    }
                } else {
                    self.set_x(self.x() + 1);
                }
            }
            Direction::North => {
                if self.y() == 0 {
                    // change chunk
                    if self.chunk_y() > 0 {
                        self.set_chunk_y(self.chunk_y() - 1);
                        self.set_y(CHUNK_SIZE_Y - 1);
                    }
                } else {
                    self.set_y(self.y() - 1);
                }
            }
            Direction::South => {
                if self.y() >= CHUNK_SIZE_Y - 1 {
                    // change chunk
                    if self.chunk_y() < CHUNK_COUNT_SQRT - 1 {
                        self.set_chunk_y(self.chunk_y() + 1);
                        self.set_y(0);
                    }
                } else {
                    self.set_y(self.y() + 1);
                }
            }
            Direction::Up => {
                if self.z() < CHUNK_SIZE_Z - 1 {
                    self.set_z(self.z() + 1);
                }
            }
            Direction::Down => {
                if self.z() > 0 {
                    self.set_z(self.z() - 1);
                }
            }
        }
    }
}

pub struct Curses(pub EasyCurses);

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MapCursor(pub Position);

// boi
unsafe impl Send for Curses {}
// Garanteed by the system execution scheduler
unsafe impl Sync for Curses {}

//pub const GOAL: Pos = Pos(4095, 4095);

lazy_static! {
    static ref COLOR_NORMAL: easycurses::ColorPair =
        easycurses::ColorPair::new(Color::White, Color::Black);
    static ref COLOR_EDGE: easycurses::ColorPair =
        easycurses::ColorPair::new(Color::Yellow, Color::Black);
    static ref COLOR_TITLE: easycurses::ColorPair =
        easycurses::ColorPair::new(Color::Red, Color::White);
    static ref COLOR_DEBUG: easycurses::ColorPair =
        easycurses::ColorPair::new(Color::Blue, Color::White);
}

#[derive(Clone, Default, Debug)]
pub struct RenderInfo {
    screen_width: u32,
    screen_height: u32,
}

impl RenderInfo {
    pub fn render_width(&self) -> u32 {
        self.screen_width - MAIN_AREA_MARGIN_LEFT - MAIN_AREA_MARGIN_RIGHT
    }
    pub fn render_height(&self) -> u32 {
        self.screen_height - MAIN_AREA_MARGIN_TOP - MAIN_AREA_MARGIN_BOTTOM
    }
    pub fn maximum_positions(&self) -> (u32, u32) {
        let (xmax, ymax) = (
            min(
                self.screen_width - MAIN_AREA_MARGIN_RIGHT,
                MAIN_AREA_MARGIN_LEFT + CHUNK_SIZE_X as u32,
            ),
            min(
                self.screen_height - MAIN_AREA_MARGIN_BOTTOM,
                MAIN_AREA_MARGIN_TOP + CHUNK_SIZE_Y as u32,
            ),
        );
        (xmax, ymax)
    }
    pub fn map_offsets(&self, cursor: &MapCursor) -> (u32, u32) {
        // Try to keep the cursor centered
        // 0 <= offset <= end - render_size
        let map_offset = (
            min(
                max(0, cursor.0.x() as i32 - (self.render_width() >> 1) as i32),
                max(0, CHUNK_SIZE_X as i32 - self.render_width() as i32),
            ) as u32,
            //min(max(0, layered_cursor.1 as i32 - ((layered_y_stop - layered_y_start) >> 1) as i32), square_count as i32 - render_height as i32) as u32,
            min(
                max(0, cursor.0.y() as i32 - (self.render_height() >> 1) as i32),
                max(0, CHUNK_SIZE_Y as i32 - self.render_height() as i32),
            ) as u32,
        );
        map_offset
    }
    pub fn position_to_main_area(
        &self,
        cursor: &MapCursor,
        position: (u32, u32),
    ) -> Option<(u32, u32)> {
        let offsets = self.map_offsets(cursor);
        let (xmax, ymax) = self.maximum_positions();
        let x_pos = offsets.0 as i32 + position.0 as i32 + MAIN_AREA_MARGIN_LEFT as i32;
        let y_pos = offsets.1 as i32 + position.1 as i32 + MAIN_AREA_MARGIN_TOP as i32;
        if x_pos >= 0 && y_pos >= 0 && (x_pos as u32) < xmax && (y_pos as u32) < ymax {
            Some((x_pos as u32, y_pos as u32))
        } else {
            None
        }
    }
}

fn main() {
    let mut world = World::default();

    world.initialize::<Entities>();
    // TODO remove this init once we split the dispatchers
    world.initialize::<Components<Controlled>>();
    world.initialize::<RNG>();

    // client dispatcher
    // receive events from server and apply to the single loaded chunk we see
    // read inputs
    // add inputs to event queue
    // send event to server, if current event is the same as last one and we didn't get a server
    // tick event, don't send again.
    // render screen and up
    // manage different screens and transitions
    //
    //
    // fixed update dispatcher
    // receive events from client, keeping only last received for each
    // run game logic update, move entities, apply user actions, load/unload chunks
    // send events to client updating the world

    // client side
    // only one chunk loaded
    // multiple players, one of which is you but you don't directly control, and see only in your
    // current chunk
    //
    // server side
    // multiple chunk loaded
    // multiple players, all assigned to one network connection

    let mut client_dispatcher = DispatcherBuilder::default();
    client_dispatcher = client_dispatcher.add(curses_update_render_info_system);
    client_dispatcher = client_dispatcher.add(curses_input_system);
    client_dispatcher = client_dispatcher.add(cursor_move_system);
    client_dispatcher = client_dispatcher.add(curses_render_system);
    client_dispatcher = client_dispatcher.add(entity_curses_render_system);
    client_dispatcher = client_dispatcher.add(curses_render_inventory_system);
    client_dispatcher = client_dispatcher.add(curses_render_crafting_system);
    client_dispatcher = client_dispatcher.add(curses_end_draw_system);
    client_dispatcher = client_dispatcher.add(|ev1: &mut Vec<InputEvent>| {
        ev1.clear();
        Ok(())
    });

    let client_dispatcher = client_dispatcher.build(&mut world);

    let mut logic_dispatcher = DispatcherBuilder::default();
    let logic_dispatcher = logic_dispatcher.build(&mut world);

    let mut engine = Engine::<GameData, _>::new(
        LoadState,
        GameData {
            world,
            render_dispatcher: client_dispatcher,
            logic_dispatcher,
        },
        |_, _| {},
        60.0,
    );
    engine.engine_loop();
}
