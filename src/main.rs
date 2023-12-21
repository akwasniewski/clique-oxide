use CliqueOxide::run;
use CliqueOxide::graph::*;
use std::thread;
use std::time::Duration;
use std::io;
fn get_snap() -> (String, usize){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut buffer = buffer.split_whitespace();
    let nodes: usize=buffer.next().unwrap().parse().unwrap();
    let edges: usize=buffer.next().unwrap().parse().unwrap();
    let mut res = "".to_string();
    for i in 0..edges{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        res.push_str(buffer.trim());
        res.push_str(" ");
    }
    (res, nodes)
}
fn main() {
    // const GRAPH_SIZE: usize = 30;
    // let mut graph = Graph::new(GRAPH_SIZE);
    // for i in 0..GRAPH_SIZE -1{
    //     graph.add_edge(i, i+1);
    // }
    // graph.add_edge(GRAPH_SIZE-1,0);
    let (snap, nodes) = get_snap();
    let mut graph = Graph::from_snap(&snap, nodes);
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

