use CliqueOxide::run;
use crate::graph::Graph;

mod graph;
fn main() {
    let graph = Graph::new(5);
    //graph.draw();
    let(vertices, indices) = graph.get();
    pollster::block_on(run(vertices, indices));
}

