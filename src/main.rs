// #![allow(unused)]

use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

// The derive macro brings a lot of boilerplate for the Component
// so we don't have to repeat us over and over again.
// For example:
// `impl Component for Position` and for Renderable and so on.
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

// LeftMover likes to go left.
#[derive(Component, Debug)]
struct LeftMover {}

struct State {
    // A world is an ECS (Entity Control System). It gathers the data for each
    // entity or component and does something with it.
    ecs: World,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

// xy_idx returns a `vec` of tiles that represent a simple map.
pub fn xy_idx(x: i32, y: i32) -> usize {
    // Multiples the `y` position by the map width (80) and adds `x`.
    // This guarantees one tile per location.
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    // `vec!` takes in 2 parameters, what and how many.
    // Here we want a vector of 4000 `TileType::Floor`.
    let mut map = vec![TileType::Floor; 80 * 50];

    // building walls at the edge of the screen
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    // The `_` means we don't actually care about the value for `i`.
    // Rust would otherwise scream about an unused variable.
    // We place 400 random walls where there isn't the player spawn.
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // We need write access to the Position and the Player structures.
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    //
    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

// Player movement
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            // Vim bindings
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            // Arrow keys
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear the screen
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // We are iterating over each position and renderable and return a
        // tuple containing Position and Renderable components as pos and render.
        // The `.join` passes both in and guarantees both go to the same instance.
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

struct LeftWalker {}

// Systems are a way to contain component logic together and
// have them run indepenently.
//
// We are implementing Specs `System` train on our `LeftWalker` struct.
impl<'a> System<'a> for LeftWalker {
    // SystemData tells Specs what the System requires.
    // Here SystemData needs Read Access to the `LeftMover` component
    // and Write Access to `Position` component.
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    // run is the actual trait implementation required by the `impl System`
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50().with_title("Rustlike").build()?;

    let mut gs = State { ecs: World::new() };

    // Here we tell our `World` to take a look at everything we gave it, namely
    // everything that implements a `Component`, and create storages for those.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.insert(new_map());

    gs.ecs
        // We create an entity, it's like an identification number, and
        // tell our ECS that this new entity exists.
        .create_entity()
        // Then we give the entity any combination of components we want by
        // using `.with()`.
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('???'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    rltk::main_loop(context, gs)
}
