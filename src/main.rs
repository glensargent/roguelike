use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
#[macro_use]
extern crate specs_derive;

// experimental roguelike game using ecs...

// Create game state struct with the ecs world
struct State {
    ecs: World
}
// Create the main tick method
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        // get component from ecs storage
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
// Create ecs component for unit positions
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}
// Create ecs component for renderable components
#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

fn main() {
    // import RltkBuilder in to scope
    use rltk::RltkBuilder;
    // create base game context
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike")
        .build();

    // initialize game state
    let mut gs = State{
        ecs: World::new()
    };

    // tell ecs about the components we have created
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();


    // create main entity
    gs.ecs.create_entity()
    .with(Position {x: 40, y: 25})
    .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
    }).build();

    // create alternative entities
    for i in 0..10 {
        gs.ecs.create_entity()
            .with(Position {x: i * 7, y: 20})
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            }).build();
    }

    // run the core loop
    rltk::main_loop(context, gs);
}