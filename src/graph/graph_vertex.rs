use std::collections::{HashSet};
use rand::Rng;
pub (crate) struct GraphVertex{
    pub (crate) label: i32,
    pub (crate) position: [f32; 3],
    pub (crate) color: [f32; 3],
    pub (crate) connections: HashSet<u32>,
}
impl GraphVertex{
    pub(crate) fn new(label: i32) -> Self{
        let mut rng = rand::thread_rng();
        let connections=HashSet::new();
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