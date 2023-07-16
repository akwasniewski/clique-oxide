use CliqueOxide::run;
use crate::graph::Graph;

mod graph;
fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 3);
    graph.add_edge(1, 4);
    graph.add_edge(4, 2);
    graph.add_edge(3, 1);
    //graph.draw();
    let(vertices, indices) = graph.get();
    pollster::block_on(run(vertices, indices));
}

