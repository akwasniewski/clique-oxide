mod graph_vertex;

use std::vec::Vec;
use CliqueOxide::visual_vertex::VisualVertex;
use crate::graph::graph_vertex::GraphVertex;
pub(crate) struct Graph{
    pub size: u32,
    pub vertices: Vec<GraphVertex>,
    vertex_size: f32,
    edge_width: f32,
}
impl Graph{
    pub fn new(size: u32) -> Self{
        let mut vertices=Vec::new();
        let mut vertex_size:f32=5.0/100.0;
        for i in 0..size{
            vertices.push(GraphVertex::new(i as i32));
        }
        let mut edge_width:f32=5.0/1000.0;
        Self{
            size,
            vertices,
            vertex_size,
            edge_width,
        }
    }
    pub fn adjust_positions(&mut self){
        let a:f32=2000.0*2000.0;
        let threshold = 100000;
        let graph_size=self.vertices.len();
        let k: f32 =(a).sqrt()/(graph_size as f32);
        let fr = |x: f32| -> f32 {k*k/x};
        let fa = |x: f32| -> f32 { x*x/k };
        let mut cooling: f32 = 1.0;
        let cooldown: f32 = 0.999;
        for i in 0..threshold{
            let mut forces: Vec<[f32; 2]> = Vec::with_capacity(graph_size);
            for i in 0..graph_size {
                forces.push([0.0, 0.0]);
            }
            for cur in 0..graph_size{
                for other in 0..graph_size{
                    if cur!=other{
                        if (self.vertices[cur].position[0]-self.vertices[other].position[0]).abs()<0.0001 &&
                            (self.vertices[cur].position[1]-self.vertices[other].position[1]).abs()<0.0001 {
                            if cur<other {
                                forces[cur][0]+=400.0;
                                forces[cur][1]+=400.0;
                            }
                            else{
                                forces[cur][0]-=400.0;
                                forces[cur][1]-=400.0;
                            }
                            continue;
                        }
                        let delta: f32 = ((self.vertices[cur].position[0]-self.vertices[other].position[0]).powf(2.0)+(self.vertices[cur].position[1]-self.vertices[other].position[1]).powf(2.0)).sqrt();
                        let reaction = fr(delta);
                        forces[cur][0]+=(self.vertices[cur].position[0]-self.vertices[other].position[0])/delta*reaction;
                        forces[cur][1]+=(self.vertices[cur].position[1]-self.vertices[other].position[1])/delta*reaction;
                    }
                }
                for other in &self.vertices[cur].connections{
                    let other = *other as usize;
                    if (self.vertices[cur].position[0]-self.vertices[other].position[0]).abs()<0.0001 &&
                        (self.vertices[cur].position[1]-self.vertices[other].position[1]).abs()<0.0001 {
                        continue;
                    }
                    let delta: f32 = ((self.vertices[cur].position[0]-self.vertices[other].position[0]).powf(2.0)+(self.vertices[cur].position[1]-self.vertices[other].position[1]).powf(2.0)).sqrt();
                    let attraction = fa(delta);
                    let force_x =(self.vertices[cur].position[0]-self.vertices[other].position[0])/delta*attraction;
                    let force_y =(self.vertices[cur].position[1]-self.vertices[other].position[1])/delta*attraction;
                    forces[cur][0]-= force_x;
                    forces[cur][1]-= force_y;
                    forces[other][0]+= force_x;
                    forces[other][1]+= force_y;
                }
            }
            for cur in 0..graph_size{
                let force =(forces[cur][0].powf(2.0)+forces[cur][1].powf(2.0)).sqrt();
                self.vertices[cur].position[0]+=forces[cur][0]*cooling;
                self.vertices[cur].position[0] = f32::min(1000.0, f32::max(-1000.0,self.vertices[cur].position[0]));
                self.vertices[cur].position[1]+=forces[cur][1]*cooling;
                self.vertices[cur].position[1] = f32::min(1000.0, f32::max(-1000.0,self.vertices[cur].position[1]));
            }
            cooling*=cooldown;
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