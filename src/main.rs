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

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear the screen
        ctx.cls();

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
fn player_input(gs: &mut State, ctx: Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
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
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    rltk::main_loop(context, gs)
}
