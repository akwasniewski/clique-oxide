mod graph_vertex;
use std::collections::LinkedList;
use std::vec::Vec;
use CliqueOxide::visual_vertex::VisualVertex;
use crate::graph::graph_vertex::GraphVertex;
pub(crate) struct Graph{
    pub size: u32,
    pub vertices: LinkedList<GraphVertex>,
}
impl Graph{
    pub fn new(size: u32) -> Self{
        let mut vertices=LinkedList::new();
        for i in 0..size{
            vertices.push_back(GraphVertex::new(i as i32));
        }
        Self{
            size,
            vertices,
        }
    }
    pub fn draw(){

    }
    pub fn get(&self) -> (Vec<VisualVertex>, Vec<u16>){
        let mut visual_vertices: Vec<VisualVertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut cur_indice=0;
        for cur in &self.vertices{
            let pos = cur.position;
            let diameter =0.05;
            let white = [0.0, 0.0, 0.0];
            visual_vertices.push(VisualVertex{position: [pos[0]-diameter, pos[1]-diameter, 0.0], color: white} );
            visual_vertices.push(VisualVertex{position: [pos[0]-diameter, pos[1]+diameter, 0.0], color: white} );
            visual_vertices.push(VisualVertex{position: [pos[0]+diameter, pos[1]-diameter, 0.0], color: white} );
            visual_vertices.push(VisualVertex{position: [pos[0]+diameter, pos[1]+diameter, 0.0], color: white} );
            indices.push(cur_indice);//top left
            indices.push(cur_indice+3); //bottom right
            indices.push(cur_indice+1); //bottom left
            indices.push(cur_indice); //top left
            indices.push(cur_indice+2); //top right
            indices.push(cur_indice+3); //bottom right
            cur_indice+=4;
        }
        (visual_vertices, indices)
    }
}