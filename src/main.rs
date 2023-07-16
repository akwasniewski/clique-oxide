use CliqueOxide::run;
mod graph;

fn main() {
    pollster::block_on(run());
}

