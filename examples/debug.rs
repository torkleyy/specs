// Warning: this requires the `nightly` feature of Specs to be useful

extern crate specs;

use specs::prelude::*;

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

fn main() {
    let mut world = World::new();

    world.register::<Pos>();
    world.register::<Vel>();

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    world.create_entity().with(Pos(2.0)).build();

    // Without the nightly feature, this is not very useful
    println!("All entities: {:#?}", world.debug_all_entities());
}
