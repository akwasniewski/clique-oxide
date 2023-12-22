use std::collections::{HashSet};
use rand::Rng;
#[allow(unused)]
pub struct GraphVertex{
    pub (crate) label: usize,
    pub (crate) position: [f32; 3],
    pub (crate) color: [f32; 3],
    pub (crate) connections: HashSet<usize>,
}
impl GraphVertex{
    pub fn new(label: usize) -> Self{
        let mut rng = rand::thread_rng();
        let connections: HashSet<usize>=HashSet::new();
        let color=[1.0,1.0,1.0];
        let position=[rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0),0.0];
        Self{
            label,
            connections,
            color,
            position,
        }
    }
}