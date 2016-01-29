extern crate lithosphere;
use lithosphere::world::World;

// for now a super uninteresting file where I can see what's going on...
// real tests to come later.

#[test]
fn main() {
    let mut world = World::new(2000, 1000, 100, 0.01, 10);
   // world.map_height();
}
