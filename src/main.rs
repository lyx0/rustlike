#![allow(unused)]

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

// A world is an ECS (Entity Control System). It gathers the data for each
// entity or component and does something with it.
struct State {
    ecs: World,
}

fn main() {
    let mut gs = State { ecs: World::new() };
    // Here we tell our `World` to take a look at everything we gave it, namely
    // everything that implements a `Component`, and create storages for those.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
}
