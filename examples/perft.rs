use ataxx;
use std::time::Instant;

fn main() {
    let pos = ataxx::Board::from_fen("startpos");

    for i in 0..=7 {
        let start = Instant::now();
        let nodes = ataxx::perft(&pos, i);
        let duration = start.elapsed();
        let nps = nodes as f64 / duration.as_secs_f64();
        println!(
            "info depth {} nodes {} time {:?} nps {}",
            i,
            nodes,
            duration.as_millis(),
            nps as u64
        );
    }
}
