use super::world::World;

pub struct Plate {
    id: usize,
    bitmap: Vec<Vec<usize>>,
}

impl Plate {
    pub fn new(id: usize, world: &World)->Plate{
        Plate{id: id, bitmap: Vec::with_capacity(world.num_plates)}
    }
}