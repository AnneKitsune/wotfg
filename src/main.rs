//use pathfinding::directed::astar;
use easycurses::constants::acs;
use easycurses::*;
use lazy_static::lazy_static;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Duration;

use derive_new::*;
use game_engine_core::*;
use minigene::{game_features::*, CollisionMap, Direction};
use modular_bitfield::prelude::*;
use planck_ecs::*;
use planck_ecs_bundle::*;

// originally, values were 40,40,10
// if we use values that can be divided by a power of two, its easier to store position as a single
// value.
const CHUNK_SIZE_X: u8 = 128;
const CHUNK_SIZE_Y: u8 = 128;
const CHUNK_SIZE_Z: u8 = 16;
const MAIN_AREA_MARGIN_LEFT: u32 = 0;
const MAIN_AREA_MARGIN_TOP: u32 = 4;
const MAIN_AREA_MARGIN_RIGHT: u32 = 20;
const MAIN_AREA_MARGIN_BOTTOM: u32 = 0;

// sqrt(18446744073709551615 / 128 / 128 / 16)
// or also, 2^23.
const CHUNK_COUNT_SQRT: u32 = 8388608;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Items {
    TestItemA,
    TestItemB,
    TestItemC,
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

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Air,
    Grass,
    Border,
    Bedrock,
    Tree,
}

// TODO do that, but for a tile that has bg and fg color, and a tile texture/animation.
impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Air => ' ',
            Tile::Grass => '0',
            Tile::Border => 'b',
            Tile::Bedrock => 'B',
            Tile::Tree => 'T',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub tiles: Vec<Tile>,
    // TODO
    //pub collisions: CollisionMap,
}

impl Chunk {
    pub fn new_rand() -> Self {
        let mut tiles = vec![];
        for x in 0..CHUNK_SIZE_X {
            for y in 0..CHUNK_SIZE_Y {
                for z in 0..CHUNK_SIZE_Z {
                    let mut tile = match (x + y) % 20 {
                        0..=15 => Tile::Grass,
                        16..=18 => Tile::Tree,
                        19..=20 => Tile::Air,
                        // unreachable
                        _ => Tile::Air,
                    };
                    if x == 0 || y == 0 || x == CHUNK_SIZE_X - 1 || y == CHUNK_SIZE_Y - 1 {
                        tile = Tile::Border;
                    }
                    if z == 0 {
                        tile = Tile::Bedrock;
                    }
                    tiles.push(tile);
                }
            }
        }
        Self { tiles }
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

pub fn entity_curses_render_system(
    cursor: &MapCursor,
    positions: &Components<Position>,
    rendered: &Components<Rendered>,
    render: &RenderInfo,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    for (pos, rend) in join!(&positions && &rendered) {
        let pos = pos.unwrap();
        let rend = rend.unwrap();
        if pos.chunk_x() == cursor.0.chunk_x()
            && pos.chunk_y() == cursor.0.chunk_y()
            && pos.z() == cursor.0.z()
        {
            if let Some((screen_x, screen_y)) =
                render.position_to_main_area(cursor, (pos.x() as u32, pos.y() as u32))
            {
                curses.move_rc(screen_y as i32, screen_x as i32);
                curses.set_color_pair(rend.color);
                curses.print_char(rend.render_char);
            } else {
                println!("outside of renderable area!");
            }
        }
    }
    Ok(())
}

pub fn curses_update_render_info_system(
    curses: &Option<Curses>,
    render: &mut RenderInfo,
) -> SystemResult {
    let (screen_height, screen_width) = curses.as_ref().unwrap().0.get_row_col_count();
    let (screen_height, screen_width) = (screen_height as u32, screen_width as u32);
    render.screen_width = screen_width;
    render.screen_height = screen_height;
    Ok(())
}

pub fn curses_render_system(
    cursor: &MapCursor,
    render: &RenderInfo,
    chunks: &HashMap<(u32, u32), Chunk>,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    // Tile space
    // Then screenspace
    // Then borders
    // Then ui

    // ---- Tile Space ----

    // ---- Screen Space ----

    // Clear the screen
    curses.set_color_pair(*COLOR_NORMAL);
    for y in 0..render.screen_height {
        for x in 0..render.screen_width {
            curses.move_rc(y as i32, x as i32);
            curses.print_char(' ');
        }
    }

    if render.screen_height < MAIN_AREA_MARGIN_BOTTOM + MAIN_AREA_MARGIN_TOP + 2
        || render.screen_width < MAIN_AREA_MARGIN_RIGHT + MAIN_AREA_MARGIN_LEFT + 2
    {
        curses.move_rc(0, 0);
        curses.print("Screen too small");
        return Ok(());
    }

    curses.set_color_pair(*COLOR_NORMAL);

    let map_offset = render.map_offsets(cursor);
    let (xmax, ymax) = render.maximum_positions();

    if let Some(chunk) = chunks.get(&(cursor.0.chunk_x(), cursor.0.chunk_y())) {
        // Render the map tiles and border
        for y in MAIN_AREA_MARGIN_TOP..ymax {
            for x in MAIN_AREA_MARGIN_LEFT..xmax {
                // calculated manually here because its faster
                let x_pos = map_offset.0 + x - MAIN_AREA_MARGIN_LEFT;
                let y_pos = map_offset.1 + y - MAIN_AREA_MARGIN_TOP;
                curses.move_rc(y as i32, x as i32);
                // TODO: Set tile color and char
                let pos = Position::new()
                    .with_x(x_pos as u8)
                    .with_y(y_pos as u8)
                    .with_z(cursor.0.z());
                if let Some(tile) = chunk.tiles.get(pos.position_index()) {
                    let c = char::from(*(tile));
                    curses.print_char(c);
                } else {
                    eprintln!(
                        "Missing tile at location {}, {}, {} (position index {}).",
                        x_pos,
                        y_pos,
                        cursor.0.z(),
                        pos.position_index()
                    );
                }
            }
        }
    } else {
        eprintln!("No chunk data for this chunk!");
    }

    // ---- Map visibility sides ----

    // how much you need to render > the space you have available
    let (edge_bottom, edge_top, edge_left, edge_right) = (
        CHUNK_SIZE_Y as u32 - map_offset.1
            > render.screen_height - MAIN_AREA_MARGIN_TOP - MAIN_AREA_MARGIN_BOTTOM,
        map_offset.1 > 0,
        map_offset.0 > 0,
        CHUNK_SIZE_X as u32 - map_offset.0
            > render.screen_width - MAIN_AREA_MARGIN_LEFT - MAIN_AREA_MARGIN_RIGHT,
    );

    curses.set_color_pair(*COLOR_EDGE);

    // Top Border
    if edge_top {
        for x in MAIN_AREA_MARGIN_LEFT..xmax {
            curses.move_rc(MAIN_AREA_MARGIN_TOP as i32, x as i32);
            curses.print_char('^');
        }
    }

    if edge_left {
        for y in MAIN_AREA_MARGIN_TOP..ymax {
            curses.move_rc(y as i32, MAIN_AREA_MARGIN_LEFT as i32);
            curses.print_char('<');
        }
    }

    if edge_bottom {
        for x in MAIN_AREA_MARGIN_LEFT..xmax {
            curses.move_rc(
                (render.screen_height - MAIN_AREA_MARGIN_BOTTOM - 1) as i32,
                x as i32,
            );
            curses.print_char('v');
        }
    }

    if edge_right {
        for y in MAIN_AREA_MARGIN_TOP..ymax {
            curses.move_rc(
                y as i32,
                (render.screen_width - MAIN_AREA_MARGIN_RIGHT - 1) as i32,
            );
            curses.print_char('>');
        }
    }

    // ---- UI ----

    curses.set_color_pair(*COLOR_TITLE);

    // Debug Info
    curses.move_rc(0, 0);
    curses.print("World of The Fox God V0.1A");

    curses.set_color_pair(*COLOR_DEBUG);

    curses.move_rc(1, 0);
    curses.print(format!(
        "Chunk: {},{} Position: {},{},{}",
        cursor.0.chunk_x(),
        cursor.0.chunk_y(),
        cursor.0.x(),
        cursor.0.y(),
        cursor.0.z(),
    ));

    // Sidebar Test
    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(4, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("Some Things");
    curses.move_rc(5, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("And More");

    // Map Cursor

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(
        (cursor.0.y() as u32 + MAIN_AREA_MARGIN_TOP - map_offset.1) as i32,
        (cursor.0.x() as u32 + MAIN_AREA_MARGIN_LEFT - map_offset.0) as i32,
    );
    curses.print_char(acs::block());

    Ok(())
}

pub fn curses_render_inventory_system(
    controlled: &Components<Controlled>,
    inventories: &Components<Inventory<Items, (), ()>>,
    render: &RenderInfo,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(6, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
    curses.print("=== Inventory ===");
    let mut y = 7;
    for (_, inv) in join!(&controlled && &inventories) {
        for item in inv.as_ref().unwrap().content.iter() {
            if item.is_some() {
                curses.move_rc(y, (render.screen_width - MAIN_AREA_MARGIN_RIGHT) as i32);
                curses.print(format!(
                    "{:?} x{}",
                    item.as_ref().unwrap().key,
                    item.as_ref().unwrap().quantity
                ));
                y += 1;
            }
        }
    }
    Ok(())
}

pub fn curses_end_draw_system(curses: &mut Option<Curses>) -> SystemResult {
    // Render
    curses.as_mut().unwrap().0.refresh();
    Ok(())
}

fn cursor_move_system(input_ev: &mut Vec<InputEvent>, cursor: &mut MapCursor) -> SystemResult {
    for ev in input_ev {
        let new = match ev {
            InputEvent::MoveUp => cursor.0.move_towards(Direction::North),
            InputEvent::MoveDown => cursor.0.move_towards(Direction::South),
            InputEvent::MoveRight => cursor.0.move_towards(Direction::East),
            InputEvent::MoveLeft => cursor.0.move_towards(Direction::West),
            InputEvent::LayerUp => cursor.0.move_towards(Direction::Up),
            InputEvent::LayerDown => cursor.0.move_towards(Direction::Down),
            _ => continue,
        };
    }
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputEvent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    LayerUp,
    LayerDown,
    Cancel,
    Accept,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keymap {
    pub map: HashMap<Input, InputEvent>,
}

impl Default for Keymap {
    fn default() -> Self {
        Keymap {
            map: [
                (Input::Character('h'), InputEvent::MoveLeft),
                (Input::Character('l'), InputEvent::MoveRight),
                (Input::Character('j'), InputEvent::MoveDown),
                (Input::Character('k'), InputEvent::MoveUp),
                (Input::Character(';'), InputEvent::LayerDown),
                (Input::Character('.'), InputEvent::LayerUp),
                (Input::Character('\n'), InputEvent::Accept),
                (Input::Character('\u{1b}'), InputEvent::Cancel), // Escape
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

fn curses_input_system(
    keymap: &Keymap,
    input_ev: &mut Vec<InputEvent>,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;
    while let Some(input) = curses.get_input() {
        if let Some(ev) = keymap.map.get(&input) {
            input_ev.push(*ev);
        }
    }
    Ok(())
}

pub struct GameData {
    pub dispatcher: Dispatcher,
    pub world: World,
}

pub struct InitState;

impl State<GameData> for InitState {
    fn on_start(&mut self, data: &mut GameData) {
        println!("Game started!");
        let entity = data.world.get_mut::<Entities>().unwrap().create();
        /*data.world
        .get_mut::<Components<_>>()
        .unwrap()
        .insert(entity, Pos(1, 1));*/

        let mut curses = EasyCurses::initialize_system().expect("Failed to start ncurses.");
        curses.set_input_mode(InputMode::Character);
        curses.set_keypad_enabled(true);
        curses.set_echo(false);
        curses.set_cursor_visibility(CursorVisibility::Invisible);
        curses.set_input_timeout(TimeoutMode::Immediate);
        #[cfg(unix)]
        unsafe {
            // TODO remove uses of escape key and then this dependency
            ncurses::ll::set_escdelay(0)
        };

        curses.refresh();
        *data.world.get_mut::<Option<Curses>>().unwrap() = Some(Curses(curses));
    }

    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        data.dispatcher
            .run_seq(&mut data.world)
            .expect("Failed to run systems.");
        data.world.maintain();

        StateTransition::None
    }
}

fn main() {
    let mut world = World::default();

    world.initialize::<Entities>();
    // TODO remove this init once we split the dispatchers
    world.initialize::<Components<Controlled>>();

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

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher = dispatcher.add(curses_update_render_info_system);
    dispatcher = dispatcher.add(curses_input_system);
    dispatcher = dispatcher.add(cursor_move_system);
    dispatcher = dispatcher.add(curses_render_system);
    dispatcher = dispatcher.add(entity_curses_render_system);
    dispatcher = dispatcher.add(curses_render_inventory_system);
    dispatcher = dispatcher.add(curses_end_draw_system);
    dispatcher = dispatcher.add(|ev1: &mut Vec<InputEvent>| {
        ev1.clear();
        Ok(())
    });
    let dispatcher = dispatcher.build(&mut world);

    world
        .get_mut::<HashMap<(u32, u32), Chunk>>()
        .unwrap()
        .insert((0, 0), Chunk::new_rand());
    world
        .get_mut::<HashMap<(u32, u32), Chunk>>()
        .unwrap()
        .insert((0, 1), Chunk::new_rand());
    world
        .get_mut::<HashMap<(u32, u32), Chunk>>()
        .unwrap()
        .insert((1, 0), Chunk::new_rand());
    world
        .get_mut::<HashMap<(u32, u32), Chunk>>()
        .unwrap()
        .insert((1, 1), Chunk::new_rand());

    let mut inv = Inventory::<Items, (), ()>::new_dynamic(0, 9999);
    inv.insert(ItemInstance::new(Items::TestItemA, 1))
        .expect("Failed to insert init item into inventory.");
    inv.get_mut(0).unwrap().quantity = 2;
    inv.insert(ItemInstance::new(Items::TestItemB, 1))
        .expect("Failed to insert init item into inventory.");
    inv.insert(ItemInstance::new(Items::TestItemC, 1))
        .expect("Failed to insert init item into inventory.");

    let mut item_defs = ItemDefinitions::from(vec![
        ItemDefinition::<Items, (), ()>::new(
            Items::TestItemA,
            (),
            "Test Item A".to_string(),
            "test_item_a".to_string(),
            "A simple test item.".to_string(), 
            None,
            None,
        ),
        ItemDefinition::<Items, (), ()>::new(
            Items::TestItemB,
            (),
            "Test Item B".to_string(),
            "test_item_b".to_string(),
            "A simple test item.".to_string(), 
            None,
            None,
        ),
        ItemDefinition::<Items, (), ()>::new(
            Items::TestItemC,
            (),
            "Test Item C".to_string(),
            "test_item_c".to_string(),
            "A simple test item.".to_string(), 
            None,
            None,
        ),
    ]);

    let player = world.get_mut::<Entities>().unwrap().create();
    world.get_mut::<Components<_>>().unwrap().insert(
        player,
        Position::new()
            .with_x(0)
            .with_y(0)
            .with_z(0)
            .with_chunk_x(0)
            .with_chunk_y(0),
    );
    world
        .get_mut::<Components<_>>()
        .unwrap()
        .insert(player, Controlled);
    world
        .get_mut::<Components<_>>()
        .unwrap()
        .insert(player, Rendered::new('P', *COLOR_TITLE, None));
    world
        .get_mut::<Components<_>>()
        .unwrap()
        .insert(player, inv);

    let mut engine =
        Engine::<GameData, _>::new(InitState, GameData { world, dispatcher }, |_, _| {}, 60.0);
    engine.engine_loop();
}
