use rltk::{GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // clear screen
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    // We're setting a new variable named gs (game state) as a copy
    // of the `State` struct.
    let gs = State {};
    rltk::main_loop(context, gs)
}
