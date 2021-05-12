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
const CHUNK_SIZE_X: u32 = 64;
const CHUNK_SIZE_Y: u32 = 64;
const CHUNK_SIZE_Z: u32 = 16;
const MAIN_AREA_OFFSET_X: u32 = 0;
const MAIN_AREA_OFFSET_Y: u32 = 4;
const UI_SIZE_X: u32 = 20;
const UI_SIZE_Y: u32 = 0;

#[bitfield]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Position {
    chunk_x: B24, // 16777216
    chunk_y: B24, // 16777216
    x: B6,        // 64
    y: B6,        // 64
    z: B4,        // 16
}

impl Position {
    pub fn chunk_index(&self) -> u64 {
        ((self.chunk_x() as u64) << 24) & (self.chunk_y() as u64)
    }
    /// Returns the position inside the chunk as a single number
    pub fn position_index(&self) -> u16 {
        ((self.x() as u16) << 10) & ((self.y() as u16) << 4) & (self.z() as u16)
    }
}

enum Tile {
    Air,
    Grass,
}

pub struct Chunk {
    tiles: Vec<Tile>,
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

fn curses_render_system(cursor: &MapCursor, curses: &mut Option<Curses>) -> SystemResult {
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
        min(screen_width as u32 - UI_SIZE_X, CHUNK_SIZE_X),
        min(screen_height as u32 - UI_SIZE_Y, CHUNK_SIZE_Y),
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

    if screen_height < UI_SIZE_Y + 2 || screen_width < UI_SIZE_X + 2 {
        curses.move_rc(0, 0);
        curses.print("Screen too small");
        return Ok(());
    }

    curses.set_color_pair(*COLOR_NORMAL);

    // Render the map tiles and border
    for y in MAIN_AREA_OFFSET_Y..ymax {
        for x in MAIN_AREA_OFFSET_X..xmax {
            let x_pos = x;
            let y_pos = y;
            curses.move_rc(y_pos as i32, x_pos as i32);
            // TODO: Set tile color and char
            curses.print_char('0');
        }
    }

    // ---- Map Borders ----

    // how much you need to render > the space you have available
    let (edge_bottom, edge_top, edge_left, edge_right) = (
        CHUNK_SIZE_Y - map_offset.1 > screen_height - MAIN_AREA_OFFSET_Y - UI_SIZE_Y,
        map_offset.1 > 0,
        map_offset.0 > 0,
        CHUNK_SIZE_X - map_offset.0 > screen_width - MAIN_AREA_OFFSET_X - UI_SIZE_X,
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

fn cursor_move_system(input_ev: &mut Vec<InputEvent>, cursor: &mut MapCursor) -> SystemResult {
    for ev in input_ev {
        let new = match ev {
            InputEvent::MoveUp => (Some(cursor.0.x()), cursor.0.y().checked_sub(1)),
            InputEvent::MoveDown => (Some(cursor.0.x()), cursor.0.y().checked_add(1)),
            InputEvent::MoveRight => (cursor.0.x().checked_add(1), Some(cursor.0.y())),
            InputEvent::MoveLeft => (cursor.0.x().checked_sub(1), Some(cursor.0.y())),
            _ => continue,
        };
        if let (Some(new_x), Some(new_y)) = new {
            cursor.0.set_x(new_x);
            cursor.0.set_y(new_y);
        }
    }
    Ok(())
}

fn layer_visibility_change_system(
    input_ev: &mut Vec<InputEvent>,
    cursor: &mut MapCursor,
) -> SystemResult {
    for ev in input_ev {
        match ev {
            InputEvent::LayerUp => {
                if cursor.0.z() < 15 {
                    cursor.0.set_z(cursor.0.z() + 1);
                }
            }
            InputEvent::LayerDown => {
                if cursor.0.z() > 0 {
                    cursor.0.set_z(cursor.0.z() - 1);
                }
            }
            _ => continue,
        }
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
    dispatcher = dispatcher.add(layer_visibility_change_system);
    dispatcher = dispatcher.add(cursor_move_system);
    dispatcher = dispatcher.add(curses_render_system);
    dispatcher = dispatcher.add(|ev1: &mut Vec<InputEvent>| {
        ev1.clear();
        Ok(())
    });
    let dispatcher = dispatcher.build(&mut world);

    let mut engine =
        Engine::<GameData, _>::new(InitState, GameData { world, dispatcher }, |_, _| {}, 60.0);
    engine.engine_loop();
}
