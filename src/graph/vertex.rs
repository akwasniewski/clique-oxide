use std::collections::LinkedList;
pub (crate) struct Vertex{
    label: i32,
    position: (f32, f32, f32),
    color: (f32, f32, f32),
    connections: LinkedList<u32>,
}
impl Vertex{
    pub(crate) fn new(label: i32) -> Self{
        let connections=LinkedList::new();
        let color=(0.0,0.0,0.0);
        let position=(0.0,0.0,0.0);
        Self{
            label,
            connections,
            color,
            position,
        }
    }
}