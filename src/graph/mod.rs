pub mod graph_vertex;
pub mod builders;
pub(crate) mod visual_vertex;

mod setters;
mod algorithm;
use rand::Rng;
use std::vec::Vec;
use crate::graph::graph_vertex::GraphVertex;
use crate::graph::visual_vertex::VisualVertex;
pub struct Graph{
    pub size: usize,
    pub vertices: Vec<GraphVertex>,
    sqrt_size: f32,
    pow_density: f32,
    vertex_size: f32,
    edge_width: f32,
    sim_cooldown: f32,
    sim_temperature: f32,
    vertex_density: f32,
}
impl Graph{
    #[allow(unused)]
    pub fn new(size: usize) -> Self{
        let mut vertices=Vec::new();
        let sqrt_size = (size as f32).sqrt();
        //setting default display options
        let mut vertex_size:f32=1.0/100.0;
        let mut edge_width:f32=5.0/1000.0;
        //setting default simulation options
        let mut sim_cooldown:f32=0.999;
        let mut sim_temperature=0.003;

        let vertex_density:f32= ((2 * 2) as f32).sqrt()/(size as f32);
        let pow_density = vertex_density*vertex_density;
        for i in 0..size{
            vertices.push(GraphVertex::new(i));
        }
        Self{
            size,
            vertices,
            vertex_size,
            edge_width,
            sim_cooldown,
            sim_temperature,
            vertex_density,
            pow_density,
            sqrt_size,
        }
    }
    fn generate_visual_vertices(&self, visual_vertices: &mut Vec<VisualVertex>, indices: &mut Vec<u16>, cur_indice: &mut u16){
        for cur in &self.vertices{
            let pos = [cur.position[0], cur.position[1], cur.position[2]];
            let color = cur.color;
            visual_vertices.push(VisualVertex{position: [pos[0]-self.vertex_size, pos[1]-self.vertex_size, pos[2]], color } );
            visual_vertices.push(VisualVertex{position: [pos[0]-self.vertex_size, pos[1]+self.vertex_size, pos[2]], color } );
            visual_vertices.push(VisualVertex{position: [pos[0]+self.vertex_size, pos[1]-self.vertex_size, pos[2]], color } );
            visual_vertices.push(VisualVertex{position: [pos[0]+self.vertex_size, pos[1]+self.vertex_size, pos[2]], color } );
            indices.push(*cur_indice);//top left
            indices.push(*cur_indice+3); //bottom right
            indices.push(*cur_indice+1); //bottom left
            indices.push(*cur_indice); //top left
            indices.push(*cur_indice+2); //top right
            indices.push(*cur_indice+3); //bottom right
            *cur_indice+=4;
        }
    }
    fn generate_edges(&self, visual_vertices: &mut Vec<VisualVertex>, indices: &mut Vec<u16>, cur_indice: &mut u16){
        for cur in &self.vertices {
            let edge_color: [f32; 3] = [0.0, 0.0, 0.0];
            let pos = [cur.position[0], cur.position[1], cur.position[2]];
            for con in &cur.connections {
                let con_pos = self.vertices[*con as usize].position;
                let con_pos = [con_pos[0], con_pos[1], con_pos[2]];
                let dx = con_pos[0] - pos[0];
                let dy = con_pos[1] - pos[1];
                let l = dx.hypot(dy);
                let u = dx * self.edge_width * 0.5 / l;
                let v = dy * self.edge_width * 0.5 / l;
                visual_vertices.push(VisualVertex { position: [pos[0] + v, pos[1] - u, 0.0], color: edge_color });
                visual_vertices.push(VisualVertex { position: [pos[0] - v, pos[1] + u, 0.0], color: edge_color });
                visual_vertices.push(VisualVertex { position: [con_pos[0] - v, con_pos[1] + u, 0.0], color: edge_color });
                visual_vertices.push(VisualVertex { position: [con_pos[0] + v, con_pos[1] - u, 0.0], color: edge_color });
                indices.push(*cur_indice + 2);
                indices.push(*cur_indice + 1);
                indices.push(*cur_indice + 0);
                indices.push(*cur_indice + 2);
                indices.push(*cur_indice + 0);
                indices.push(*cur_indice + 3);
                *cur_indice += 4;
            }
        }
    }
}