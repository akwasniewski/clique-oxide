use crate::graph::*;
fn are_collinear(v1: &GraphVertex, v2: &GraphVertex, v3: &GraphVertex)->bool{
    ((v3.position[1] - v2.position[1]) *
        (v2.position[0] - v1.position[0]) -
        (v2.position[1] - v1.position[1])
            * (v3.position[0] - v2.position[1])).abs()<0.00001
}
impl Graph{
     fn handle_collinearities(&mut self){
        let graph_size=self.vertices.len();
        for cur in 0..graph_size{
            for other in 0..graph_size{
                if cur==other {continue;}
                for another in 0..graph_size{
                    if cur==another || other==another {continue;}
                    if are_collinear(&self.vertices[cur], &self.vertices[other], &self.vertices[another]){
                        //colinear
                        let mut rng = rand::thread_rng();
                        self.vertices[cur].position[0]+=rng.gen_range(-0.001..0.001);
                        self.vertices[cur].position[1]+=rng.gen_range(-0.001..0.001);
                        self.vertices[other].position[0]+=rng.gen_range(-0.001..0.001);
                        self.vertices[other].position[1]+=rng.gen_range(-0.001..0.001);
                        self.vertices[another].position[0]+=rng.gen_range(-0.001..0.001);
                        self.vertices[another].position[1]+=rng.gen_range(-0.001..0.001);
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
                            forces[cur][0] += 0.4;
                            forces[cur][1] += 0.4;
                        } else {
                            forces[cur][0] -= 0.4;
                            forces[cur][1] -= 0.4;
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
            let connections = self.vertices[cur].connections.len();
            for other in &self.vertices[cur].connections{
                let other = *other as usize;
                if (self.vertices[cur].position[0]-self.vertices[other].position[0]).abs()<0.001 &&
                    (self.vertices[cur].position[1]-self.vertices[other].position[1]).abs()<0.001 {
                    continue;
                }
                let delta: f32 = ((self.vertices[cur].position[0]-self.vertices[other].position[0]).powf(2.0)+(self.vertices[cur].position[1]-self.vertices[other].position[1]).powf(2.0)).sqrt();
                let attraction = self.f_attr(delta)/(connections*connections) as f32;
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
        self.pow_density*self.sqrt_size/delta
    }
    fn f_attr(&self, delta: f32)->f32{
        delta*delta/self.vertex_density
    }
    pub (crate) fn adjust_positions(&mut self) ->bool {
        if self.sim_temperature<0.00001{
            return false;
        }
        let mut rng = rand::thread_rng();
        let graph_size = self.vertices.len();
        let mut forces: Vec<[f32; 2]> = Vec::with_capacity(graph_size);
        for _ in 0..graph_size {
            forces.push([0.0, 0.0]);
        }
        if self.sim_temperature>0.0001 && rng.gen_range(0..20) == 0 {
            self.handle_collinearities();
        }
        self.calculate_repulsion(&mut forces);
        self.calculate_attraction(&mut forces);
        for cur in 0..graph_size {
            self.vertices[cur].position[0] += forces[cur][0] * self.sim_temperature;
            self.vertices[cur].position[0] = f32::min(0.95, f32::max(-0.95, self.vertices[cur].position[0]));
            self.vertices[cur].position[1] += forces[cur][1] * self.sim_temperature;
            self.vertices[cur].position[1] = f32::min(0.95, f32::max(-0.95, self.vertices[cur].position[1]));
        }
        self.sim_temperature *= self.sim_cooldown;
        return true;
    }
}