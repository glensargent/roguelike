use rltk::{Rltk, GameState, Console};

// Create game state struct
struct State {}
// Create the main tick method
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() {
    // import RltkBuilder in to scope
    use rltk::RltkBuilder;
    // create base game context
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike")
        .build();

    // initialize game state
    let gs = State{};

    // run the core loop
    rltk::main_loop(context, gs);
}