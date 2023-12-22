use crate::graph::*;
impl Graph{
    pub fn size(&self)->usize{
        self.size
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
    pub fn set_cooldown(&mut self, new_cooldown: f32){
        self.sim_cooldown=new_cooldown;
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
}