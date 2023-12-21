use CliqueOxide::run;
use CliqueOxide::graph::*;
use std::thread;
use std::time::Duration;
fn main() {
    const GRAPH_SIZE: usize = 30;
    let mut graph = Graph::new(GRAPH_SIZE);
    for i in 0..GRAPH_SIZE -1{
        graph.add_edge(i, i+1);
    }
    graph.add_edge(GRAPH_SIZE-1,0);

    // for i in 0..10000{
    //     graph.adjust_positions();
    // }
    // for vert in &graph.vertices{
    //     println!("{:?}", vert.position);
    // }
    //graph.draw();
    let(vertices, indices) = graph.get();
    pollster::block_on(run(graph));
}

