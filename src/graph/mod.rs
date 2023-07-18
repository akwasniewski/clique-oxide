mod graph_vertex;
use rand::Rng;
use std::vec::Vec;
use CliqueOxide::visual_vertex::VisualVertex;
use crate::graph::graph_vertex::GraphVertex;
pub(crate) struct Graph{
    pub size: u32,
    pub vertices: Vec<GraphVertex>,
    vertex_size: f32,
    edge_width: f32,
    sim_runs: i32,
    sim_cooldown: f32,
    sim_temperature: f32,
    display_length: i32,
    display_height: i32,
    vertex_density: f32,
}
pub (crate) fn are_collinear(v1: &GraphVertex, v2: &GraphVertex, v3: &GraphVertex)->bool{
    ((v3.position[1] - v2.position[1]) *
        (v2.position[0] - v1.position[0]) -
        (v2.position[1] - v1.position[1])
            * (v3.position[0] - v2.position[1])).abs()<0.0001
}
impl Graph{
    pub fn new(size: u32) -> Self{ let mut vertices=Vec::new();

        //setting default display options
        let mut vertex_size:f32=5.0/100.0;
        let mut edge_width:f32=5.0/1000.0;

        //setting default simulation options
        let mut sim_runs:i32=100000;
        let mut sim_cooldown:f32=0.999;
        let mut sim_temperature=1.0;
        let mut display_length:i32=2000;
        let mut display_height:i32=2000;
        let mut vertex_density:f32= ((display_height * display_length) as f32).sqrt()/(size as f32);
        for i in 0..size{
            vertices.push(GraphVertex::new(i as i32));
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
            vertex_density
        }
    }
    pub fn handle_collinearities(&mut self){
        let graph_size=self.vertices.len();
        for cur in 0..graph_size{
            for other in 0..graph_size{
                if cur==other {continue;}
                for another in 0..graph_size{
                    if cur==another || other==another {continue;}
                    if are_collinear(&self.vertices[cur], &self.vertices[other], &self.vertices[another]){
                        //colinear
                        let mut rng = rand::thread_rng();
                        self.vertices[cur].position[0]+=rng.gen_range(-1.0..1.0);
                        self.vertices[cur].position[1]+=rng.gen_range(-1.0..1.0);
                        self.vertices[other].position[0]+=rng.gen_range(-1.0..1.0);
                        self.vertices[other].position[1]+=rng.gen_range(-1.0..1.0);
                        self.vertices[another].position[0]+=rng.gen_range(-1.0..1.0);
                        self.vertices[another].position[1]+=rng.gen_range(-1.0..1.0);
                    }
                }
            }
        }
    }
    fn calculate_repulsion(&self, forces: &mut Vec<[f32; 2]>){
        let graph_size=self.vertices.len();
        for cur in 0..graph_size {
            for other in 0..graph_size {
                if cur != other {
                    if (self.vertices[cur].position[0] - self.vertices[other].position[0]).abs() < 0.0001 &&
                        (self.vertices[cur].position[1] - self.vertices[other].position[1]).abs() < 0.0001 {
                        if cur < other {
                            forces[cur][0] += 400.0;
                            forces[cur][1] += 400.0;
                        } else {
                            forces[cur][0] -= 400.0;
                            forces[cur][1] -= 400.0;
                        }
                        continue;
                    }
                    let delta: f32 = ((self.vertices[cur].position[0] - self.vertices[other].position[0]).powf(2.0) + (self.vertices[cur].position[1] - self.vertices[other].position[1]).powf(2.0)).sqrt();
                    let reaction = self.f_rep(delta);
                    forces[cur][0] += (self.vertices[cur].position[0] - self.vertices[other].position[0]) / delta * reaction;
                    forces[cur][1] += (self.vertices[cur].position[1] - self.vertices[other].position[1]) / delta * reaction;
                }
            }
        }
    }
    fn calculate_attraction(&self, forces: &mut Vec<[f32; 2]>) {
        let graph_size=self.vertices.len();
        for cur in 0..graph_size{
            for other in &self.vertices[cur].connections{
                let other = *other as usize;
                if (self.vertices[cur].position[0]-self.vertices[other].position[0]).abs()<0.0001 &&
                    (self.vertices[cur].position[1]-self.vertices[other].position[1]).abs()<0.0001 {
                    continue;
                }
                let delta: f32 = ((self.vertices[cur].position[0]-self.vertices[other].position[0]).powf(2.0)+(self.vertices[cur].position[1]-self.vertices[other].position[1]).powf(2.0)).sqrt();
                let attraction = self.f_attr(delta);
                let force_x =(self.vertices[cur].position[0]-self.vertices[other].position[0])/delta*attraction;
                let force_y =(self.vertices[cur].position[1]-self.vertices[other].position[1])/delta*attraction;
                forces[cur][0]-= force_x;
                forces[cur][1]-= force_y;
                forces[other][0]+= force_x;
                forces[other][1]+= force_y;
            }
        }
    }
    fn f_rep(&self, delta: f32)->f32{
        self.vertex_density*self.vertex_density/delta
    }
    fn f_attr(&self, delta: f32)->f32{
        delta*delta/self.vertex_density
    }
    pub fn adjust_positions(&mut self){
        let graph_size=self.vertices.len();
        for i in 0..self.sim_runs{
            let mut forces: Vec<[f32; 2]> = Vec::with_capacity(graph_size);
            for i in 0..graph_size {
                forces.push([0.0, 0.0]);
            }
            if i%20==0{
                self.handle_collinearities();
            }
            self.calculate_repulsion(&mut forces);
            self.calculate_attraction(&mut forces);
            for cur in 0..graph_size{
                let force =(forces[cur][0].powf(2.0)+forces[cur][1].powf(2.0)).sqrt();
                self.vertices[cur].position[0]+=forces[cur][0]*self.sim_temperature;
                self.vertices[cur].position[0] = f32::min(1000.0, f32::max(-1000.0,self.vertices[cur].position[0]));
                self.vertices[cur].position[1]+=forces[cur][1]*self.sim_temperature;
                self.vertices[cur].position[1] = f32::min(1000.0, f32::max(-1000.0,self.vertices[cur].position[1]));
            }
            self.sim_temperature*=self.sim_cooldown;
        }
        for cur in 0..graph_size{
            self.vertices[cur].position[0]/=1000.0;
            self.vertices[cur].position[1]/=1000.0;
        }
    }
    pub fn set_vertex_size(&mut self, vertex_size: f32){
        self.vertex_size=vertex_size/100.0;
    }
    pub fn set_edge_with(&mut self, edge_width: f32){
        self.edge_width=edge_width/200.0;
    }
    pub fn add_edge(&mut self, from: u32, to: u32){
        self.vertices[from as usize].connections.insert(to);
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
            let pos = cur.position;
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
            let pos = cur.position;
            for con in &cur.connections {
                let con_pos = self.vertices[*con as usize].position;
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