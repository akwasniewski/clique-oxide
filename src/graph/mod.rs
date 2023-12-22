pub mod graph_vertex;
use rand::Rng;
use std::vec::Vec;
use crate::graph::graph_vertex::GraphVertex;
mod algorithm;
pub(crate) mod visual_vertex;
use crate::graph::visual_vertex::VisualVertex;
pub struct Graph{
    pub size: usize,
    pub vertices: Vec<GraphVertex>,
    sqrt_size: f32,
    pow_density: f32,
    vertex_size: f32,
    edge_width: f32,
    sim_runs: i32,
    sim_cooldown: f32,
    sim_temperature: f32,
    display_length: i32,
    display_height: i32,
    vertex_density: f32,
}
impl Graph{
    pub fn new(size: usize) -> Self{
        let mut vertices=Vec::new();
        let mut sqrt_size = (size as f32).sqrt();
        //setting default display options
        let mut vertex_size:f32=1.0/100.0;
        let mut edge_width:f32=5.0/1000.0;
        //setting default simulation options
        let mut sim_runs:i32=100000;
        let mut sim_cooldown:f32=0.9995;
        let mut sim_temperature=0.003;
        let mut display_length:i32=2000;
        let mut display_height:i32=2000;
        let mut vertex_density:f32= ((display_height * display_length) as f32).sqrt()/(size as f32);
        let mut pow_density = vertex_density*vertex_density;
        for i in 0..size{
            vertices.push(GraphVertex::new(i));
        }
        Self{
            size,
            vertices,
            vertex_size,
            edge_width,
            sim_runs,
            sim_cooldown,
            sim_temperature,
            display_height,
            display_length,
            vertex_density,
            pow_density,
            sqrt_size,
        }
    }
    pub fn from_snap(snap: &str, nodes: usize)->Graph{
        let split = snap.split_whitespace();
        let collected: Vec<usize> = split.map(|x| x.parse().unwrap()).collect();
        let mut res: Graph = Graph::new(nodes);
        for i in (0..collected.len()).step_by(2) {
            res.add_edge(collected[i]-1, collected[i + 1]-1);
        }
        res
    }
    pub fn set_temperature(&mut self, new_temperature: f32){
        self.sim_temperature=new_temperature;
    }
    pub fn set_vertex_size(&mut self, vertex_size: f32){
        self.vertex_size=vertex_size/100.0;
    }
    pub fn set_edge_with(&mut self, edge_width: f32){
        self.edge_width=edge_width/200.0;
    }
    pub fn add_edge(&mut self, from: usize, to: usize){
        self.vertices[from].connections.insert(to);
    }
    pub fn change_color(&mut self, edge_index: usize, color: [f32; 3]){
        self.vertices[edge_index].color=color;
    }
    pub fn get(&self) -> (Vec<VisualVertex>, Vec<u16>){
        let mut visual_vertices: Vec<VisualVertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut cur_indice=0;
        self.generate_edges(&mut visual_vertices, &mut indices, &mut cur_indice);
        self.generate_visual_vertices(&mut visual_vertices, &mut indices, &mut cur_indice);
        (visual_vertices, indices)
    }
    fn generate_visual_vertices(&self, visual_vertices: &mut Vec<VisualVertex>, indices: &mut Vec<u16>, cur_indice: &mut u16){
        for cur in &self.vertices{
            let pos = [cur.position[0]/1000.0, cur.position[1]/1000.0, cur.position[2]/1000.0];
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
    pub fn size(&self)->usize{
        self.size
    }
    fn generate_edges(&self, visual_vertices: &mut Vec<VisualVertex>, indices: &mut Vec<u16>, cur_indice: &mut u16){
        for cur in &self.vertices {
            let edge_color: [f32; 3] = [0.0, 0.0, 0.0];
            let pos = [cur.position[0]/1000.0, cur.position[1]/1000.0, cur.position[2]/1000.0];
            for con in &cur.connections {
                let con_pos = self.vertices[*con as usize].position;
                let con_pos = [con_pos[0]/1000.0, con_pos[1]/1000.0, con_pos[2]/1000.0];
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