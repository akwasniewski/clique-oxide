use CliqueOxide::run;
use crate::graph::Graph;
use std::thread;
use std::time::Duration;
mod graph;
fn main() {
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1);
    graph.add_edge(1,2);
    graph.add_edge(2,3);
    graph.add_edge(3,4);
    graph.add_edge(4,0);
    graph.add_edge(0,5);
    graph.add_edge(1,5);
    graph.add_edge(2,5);
    graph.add_edge(3,5);
    graph.add_edge(4,5);

    graph.adjust_positions();
    for vert in &graph.vertices{
        println!("{:?}", vert.position);
    }
    //graph.draw();
    let(vertices, indices) = graph.get();
    pollster::block_on(run(vertices, indices));
}

