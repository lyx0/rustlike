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

struct State {
    // A world is an ECS (Entity Control System). It gathers the data for each
    // entity or component and does something with it.
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear the screen
        ctx.cls();

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

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50().with_title("Rustlike").build()?;

    let mut gs = State { ecs: World::new() };

    // Here we tell our `World` to take a look at everything we gave it, namely
    // everything that implements a `Component`, and create storages for those.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

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
            .build();
    }

    rltk::main_loop(context, gs)
}
