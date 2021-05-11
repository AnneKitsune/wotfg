//use pathfinding::directed::astar;
use easycurses::constants::acs;
use easycurses::*;
use lazy_static::lazy_static;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Duration;

use game_engine_core::*;
use planck_ecs::*;
use planck_ecs_bundle::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub u32, pub u32);

pub struct Curses(pub EasyCurses);

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct MapCursor(pub u32, pub u32);

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct LayerVisibility(pub u32);

// boi
unsafe impl Send for Curses {}
// Garanteed by the system execution scheduler
unsafe impl Sync for Curses {}

pub const GOAL: Pos = Pos(4095, 4095);
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

fn curses_render_system(
    _positions: &Components<Pos>,
    cursor: &MapCursor,
    layer: &LayerVisibility,
    curses: &mut Option<Curses>,
) -> SystemResult {
    let curses = &mut curses.as_mut().unwrap().0;

    // Tile space
    // Then screenspace
    // Then borders
    // Then ui

    // ---- Tile Space ----

    let bitshift_factor = 9 * layer.0;
    //let tiles_per_square = 1 << 9 * layer.0;

    let mask: u32 = 0xFFFFFFFFu32 << bitshift_factor;
    let mask: u32 = mask << 9; // NECESSARY TO SPLIT
    let x_start_tile = cursor.0 & mask;
    let y_start_tile = cursor.1 & mask;
    //let x_start_tile = cursor.0 - (cursor.0 % tiles_per_square); // TODO: replace by bitmasking (this is the same as bitshifting right and then left??)
    //let y_start_tile = cursor.1 - (cursor.1 % tiles_per_square);

    //let x_end_tiles = x_start_tile + tiles_per_square;
    //let y_end_tiles = y_start_tile + tiles_per_square;

    // How many squares do we have to render for this layer?
    //let square_count = std::u32::MAX >> 9 * (3 - layer.0);
    let square_count = if layer.0 == 3 { 32 } else { 512 };

    let layered_x_start = x_start_tile >> bitshift_factor;
    let layered_y_start = y_start_tile >> bitshift_factor;
    //let layered_x_stop = layered_x_start + square_count - 1;
    //let layered_y_stop = layered_y_start + square_count - 1;
    let layered_cursor = (
        (cursor.0 >> bitshift_factor) - layered_x_start,
        (cursor.1 >> bitshift_factor) - layered_y_start,
    );
    let something = bitshift_factor;

    //println!("{:?}", x_start_tile);

    // ---- Screen Space ----

    let screen_offset = (0u32, 4u32);
    let outside_edge = (20u32, 3u32);

    let (screen_height, screen_width) = curses.get_row_col_count();
    let (screen_height, screen_width) = (screen_height as u32, screen_width as u32);
    let (xmax, ymax) = (
        min(screen_width as u32 - outside_edge.0, square_count),
        min(screen_height as u32 - outside_edge.1, square_count),
    );

    let render_width = screen_width - screen_offset.0 - outside_edge.0;
    let render_height = screen_height - screen_offset.1 - outside_edge.1;

    // Try to keep the cursor centered
    // 0 <= offset <= end - render_size
    let map_offset = (
        min(
            max(0, layered_cursor.0 as i32 - (render_width >> 1) as i32),
            max(0, square_count as i32 - render_width as i32),
        ) as u32,
        //min(max(0, layered_cursor.1 as i32 - ((layered_y_stop - layered_y_start) >> 1) as i32), square_count as i32 - render_height as i32) as u32,
        min(
            max(0, layered_cursor.1 as i32 - (render_height >> 1) as i32),
            max(0, square_count as i32 - render_height as i32),
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

    if screen_height < 12 || screen_width < 26 {
        curses.move_rc(0, 0);
        curses.print("Smol Screen...");
        return Ok(());
    }

    curses.set_color_pair(*COLOR_NORMAL);

    // Render the map tiles and border
    for y in screen_offset.1..ymax {
        for x in screen_offset.0..xmax {
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
        square_count - map_offset.1 > screen_height - screen_offset.1 - outside_edge.1,
        map_offset.1 > 0,
        map_offset.0 > 0,
        square_count - map_offset.0 > screen_width - screen_offset.0 - outside_edge.0,
    );

    curses.set_color_pair(*COLOR_EDGE);

    // Top Border
    if edge_top {
        for x in screen_offset.0..xmax {
            curses.move_rc(screen_offset.1 as i32, x as i32);
            curses.print_char('^');
        }
    }

    if edge_left {
        for y in screen_offset.1..ymax {
            curses.move_rc(y as i32, screen_offset.0 as i32);
            curses.print_char('<');
        }
    }

    if edge_bottom {
        for x in screen_offset.0..xmax {
            curses.move_rc((screen_height - outside_edge.1 - 1) as i32, x as i32);
            curses.print_char('v');
        }
    }

    if edge_right {
        for y in screen_offset.1..ymax {
            curses.move_rc(y as i32, (screen_width - outside_edge.0 - 1) as i32);
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
    curses.print(format!("Layer 0: ({},{}), Layer 1: ({},{}), Layer 2: ({},{}), Layer 3: ({},{}), Point On Map: ({},{})", 
                             cursor.0,
                             cursor.1,
                             cursor.0 >> 9,
                             cursor.1 >> 9,
                             cursor.0 >> 18,
                             cursor.1 >> 18,
                             cursor.0 >> 27,
                             cursor.1 >> 27,
                             layered_cursor.0,
                             layered_cursor.1));
    curses.move_rc(3, 0);
    curses.print(format!("Current Visible Layer: {}", layer.0));

    curses.move_rc(3, 50);
    curses.print(format!("Debug Data: {:?},{:?}", something, layered_x_start));

    // Sidebar Test
    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(4, (screen_width - outside_edge.0) as i32);
    curses.print("Some Things");
    curses.move_rc(5, (screen_width - outside_edge.0) as i32);
    curses.print("And More");

    // Map Cursor

    curses.set_color_pair(*COLOR_NORMAL);
    curses.move_rc(
        (layered_cursor.1 + screen_offset.1 - map_offset.1) as i32,
        (layered_cursor.0 + screen_offset.0 - map_offset.0) as i32,
    );
    curses.print_char(acs::block());

    // Render
    curses.refresh();
    Ok(())
}

fn cursor_move_system(
    layer: &LayerVisibility,
    input_ev: &mut Vec<InputEvent>,
    cursor: &mut MapCursor,
) -> SystemResult {
    let offset = 1 << 9 * layer.0;
    for ev in input_ev {
        let new = match ev {
            InputEvent::MoveUp => (Some(cursor.0), cursor.1.checked_sub(offset)),
            InputEvent::MoveDown => (Some(cursor.0), cursor.1.checked_add(offset)),
            InputEvent::MoveRight => (cursor.0.checked_add(offset), Some(cursor.1)),
            InputEvent::MoveLeft => (cursor.0.checked_sub(offset), Some(cursor.1)),
            _ => continue,
        };
        if let (Some(new_x), Some(new_y)) = new {
            cursor.0 = new_x;
            cursor.1 = new_y;
        }
    }
    Ok(())
}

fn layer_visibility_change_system(
    input_ev: &mut Vec<InputEvent>,
    layer: &mut LayerVisibility,
) -> SystemResult {
    for ev in input_ev {
        match ev {
            InputEvent::LayerUp => {
                if layer.0 < 3 {
                    layer.0 += 1
                }
            }
            InputEvent::LayerDown => {
                if layer.0 > 0 {
                    layer.0 -= 1
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

impl Pos {
    /*fn distance_bad(&self, other: &Pos) -> u32 {
        let xdiff = if self.0 <= other.0 {
            other.0 - self.0
        } else {
            self.0 - other.0
        };
        let ydiff = if self.1 <= other.1 {
            other.1 - self.1
        } else {
            self.1 - other.1
        };
        xdiff + ydiff
    }
    fn successors(&self) -> [(Pos, u32); 4] {
        let &Pos(x, y) = self;
        [(Pos(x+1, y), 1), (Pos(x-1, y), 1), (Pos(x, y+1), 1), (Pos(x, y-1), 1)]
    }*/
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
        data.world
            .get_mut::<Components<_>>()
            .unwrap()
            .insert(entity, Pos(1, 1));

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

    fn update(&mut self, _data: &mut GameData) -> StateTransition<GameData> {
        //println!("Hello from Amethyst!");
        StateTransition::None
    }
}

fn main() {
    let mut world = World::default();

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher = dispatcher.add(curses_input_system);
    dispatcher = dispatcher.add(layer_visibility_change_system);
    dispatcher = dispatcher.add(cursor_move_system);
    dispatcher = dispatcher.add(curses_render_system);
    dispatcher = dispatcher.add(|ev1: &mut Vec<InputEvent>| {ev1.clear(); Ok(())});
    let dispatcher = dispatcher.build(&mut world);

    let mut engine =
        Engine::<GameData, _>::new(InitState, GameData { world, dispatcher }, |_, _| {}, 60.0);
    engine.engine_loop();
}
