mod vertex;
use std::collections::LinkedList;
use crate::graph::vertex::Vertex;
struct Graph{
    size: u32,
    vertices: LinkedList<Vertex>,
}
impl Graph{
    pub fn new(size: u32) -> Self{
        let mut vertices=LinkedList::new();
        for i in 0..size{
            vertices.push_back(Vertex::new(i as i32));
        }
        Self{
            size,
            vertices,
        }
    }
}