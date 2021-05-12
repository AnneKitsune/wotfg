//use pathfinding::directed::astar;
use easycurses::constants::acs;
use easycurses::*;
use lazy_static::lazy_static;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Duration;

use game_engine_core::*;
use modular_bitfield::prelude::*;
use planck_ecs::*;
use planck_ecs_bundle::*;

// originally, values were 40,40,10
// if we use values that can be divided by a power of two, its easier to store position as a single
// value.
const CHUNK_SIZE_X: u8 = 128;
const CHUNK_SIZE_Y: u8 = 128;
const CHUNK_SIZE_Z: u8 = 16;
const MAIN_AREA_OFFSET_X: u32 = 0;
const MAIN_AREA_OFFSET_Y: u32 = 4;
const UI_SIZE_X: u32 = 20;
const UI_SIZE_Y: u32 = 0;

// sqrt(18446744073709551615 / 128 / 128 / 16)
// or also, 2^23.
const CHUNK_COUNT_SQRT: u32 = 8388608;

#[bitfield]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Position {
    chunk_x: B23, // 8388608
    chunk_y: B23, // 8388608
    x: B7,        // 128
    y: B7,        // 128
    z: B4,        // 16
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

fn curses_render_system(cursor: &MapCursor, chunks: &HashMap<(u32, u32), Chunk>, curses: &mut Option<Curses>) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    // Tile space
    // Then screenspace
    // Then borders
    // Then ui

    // ---- Tile Space ----

    // ---- Screen Space ----

    let (screen_height, screen_width) = curses.get_row_col_count();
    let (screen_height, screen_width) = (screen_height as u32, screen_width as u32);
    let (xmax, ymax) = (
        min(screen_width as u32 - UI_SIZE_X, CHUNK_SIZE_X as u32),
        min(screen_height as u32 - UI_SIZE_Y, CHUNK_SIZE_Y as u32),
    );

    let render_width = screen_width - MAIN_AREA_OFFSET_X - UI_SIZE_X;
    let render_height = screen_height - MAIN_AREA_OFFSET_Y - UI_SIZE_Y;

    // Try to keep the cursor centered
    // 0 <= offset <= end - render_size
    let map_offset = (
        min(
            max(0, cursor.0.x() as i32 - (render_width >> 1) as i32),
            max(0, CHUNK_SIZE_X as i32 - render_width as i32),
        ) as u32,
        //min(max(0, layered_cursor.1 as i32 - ((layered_y_stop - layered_y_start) >> 1) as i32), square_count as i32 - render_height as i32) as u32,
        min(
            max(0, cursor.0.y() as i32 - (render_height >> 1) as i32),
            max(0, CHUNK_SIZE_Y as i32 - render_height as i32),
        ) as u32,
    );

    // Clear the screen
    curses.set_color_pair(*COLOR_NORMAL);
    for y in 0..screen_height {
        for x in 0..screen_width {
            curses.move_rc(y as i32, x as i32);
            curses.print_char(' ');
        }
    }

    if screen_height < UI_SIZE_Y + MAIN_AREA_OFFSET_Y + 2 || screen_width < UI_SIZE_X + MAIN_AREA_OFFSET_X + 2 {
        curses.move_rc(0, 0);
        curses.print("Screen too small");
        return Ok(());
    }

    curses.set_color_pair(*COLOR_NORMAL);

    if let Some(chunk) = chunks.get(&(cursor.0.chunk_x(), cursor.0.chunk_y())) {
        // Render the map tiles and border
        for y in MAIN_AREA_OFFSET_Y..ymax {
            for x in MAIN_AREA_OFFSET_X..xmax {
                let x_pos = map_offset.0 + x;
                let y_pos = map_offset.1 + y;
                curses.move_rc(y as i32, x as i32);
                // TODO: Set tile color and char
                let pos = Position::new().with_x(x_pos as u8).with_y(y_pos as u8).with_z(cursor.0.z());
                let c = char::from(*(chunk.tiles.get(pos.position_index()).expect("Missing tile in chunk!")));
                curses.print_char(c);
            }
        }
    } else {
        eprintln!("No chunk data for this chunk!");
    }

    // ---- Map visibility sides ----

    // how much you need to render > the space you have available
    let (edge_bottom, edge_top, edge_left, edge_right) = (
        CHUNK_SIZE_Y as u32 - map_offset.1 > screen_height - MAIN_AREA_OFFSET_Y - UI_SIZE_Y,
        map_offset.1 > 0,
        map_offset.0 > 0,
        CHUNK_SIZE_X as u32 - map_offset.0 > screen_width - MAIN_AREA_OFFSET_X - UI_SIZE_X,
    );

    curses.set_color_pair(*COLOR_EDGE);

    // Top Border
    if edge_top {
        for x in MAIN_AREA_OFFSET_X..xmax {
            curses.move_rc(MAIN_AREA_OFFSET_Y as i32, x as i32);
            curses.print_char('^');
        }
    }

    if edge_left {
        for y in MAIN_AREA_OFFSET_Y..ymax {
            curses.move_rc(y as i32, MAIN_AREA_OFFSET_X as i32);
            curses.print_char('<');
        }
    }

    if edge_bottom {
        for x in MAIN_AREA_OFFSET_X..xmax {
            curses.move_rc((screen_height - UI_SIZE_Y - 1) as i32, x as i32);
            curses.print_char('v');
        }
    }

    if edge_right {
        for y in MAIN_AREA_OFFSET_Y..ymax {
            curses.move_rc(y as i32, (screen_width - UI_SIZE_X - 1) as i32);
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
    curses.move_rc(4, (screen_width - UI_SIZE_X) as i32);
    curses.print("Some Things");
    curses.move_rc(5, (screen_width - UI_SIZE_X) as i32);
    curses.print("And More");

    // Map Cursor

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(
        (cursor.0.y() as u32 + MAIN_AREA_OFFSET_Y - map_offset.1) as i32,
        (cursor.0.x() as u32 + MAIN_AREA_OFFSET_X - map_offset.0) as i32,
    );
    curses.print_char(acs::block());

    // Render
    curses.refresh();
    Ok(())
}

// TODO replace by minigene's builtin
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
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

        //println!("Hello from Amethyst!");
        StateTransition::None
    }
}

fn main() {
    let mut world = World::default();

    world.initialize::<Entities>();

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher = dispatcher.add(curses_input_system);
    dispatcher = dispatcher.add(cursor_move_system);
    dispatcher = dispatcher.add(curses_render_system);
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


    let mut engine =
        Engine::<GameData, _>::new(InitState, GameData { world, dispatcher }, |_, _| {}, 60.0);
    engine.engine_loop();
}
