use rand::Rng;
use crate::graph::*;
impl Graph{
    pub fn from_snap(snap: &str, nodes: usize)->Graph{
        let split = snap.split_whitespace();
        let collected: Vec<usize> = split.map(|x| x.parse().unwrap()).collect();
        let mut res: Graph = Graph::new(nodes);
        for i in (0..collected.len()).step_by(2) {
            res.add_edge(collected[i]-1, collected[i + 1]-1);
        }
        res
    }
    pub fn random(size: usize, avg_degree: usize)->Graph{
        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(size);
        for i in 0..graph.size(){
            for j in 0..graph.size(){
                if rng.gen_range(0..size/avg_degree) == 0 {
                    graph.add_edge(i,j);
                }
            }
            graph.change_color(i, [rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0),rng.gen_range(0.0.. 1.1)])
        }
        graph
    }
    pub fn clique(size: usize)->Graph{
        let mut graph = Graph::new(size);
        for i in 0..graph.size(){
            for j in i+1..graph.size{
                graph.add_edge(i,j);
            }
        }
        graph
    }
    pub fn cycle(size: usize)->Graph{
        let mut graph = Graph::new(size);
        for i in 0..graph.size()-1{
            graph.add_edge(i, i+1);
        }
        graph.add_edge(size-1, 0);
        graph
    }
}