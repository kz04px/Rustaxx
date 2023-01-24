use crate::board::*;

#[must_use]
pub fn perft(pos: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    } else if pos.gameover() {
        return 0;
    } else if depth == 1 {
        return pos.count_pseudomoves();
    }

    let moves = pos.pseudolegal_moves();
    let mut nodes: u64 = 0;

    for mv in &moves {
        let npos = pos.after_move(mv);
        nodes += perft(&npos, depth - 1);
    }

    if moves.is_empty() {
        let npos: Board = pos.after_pass();
        return perft(&npos, depth - 1);
    }

    nodes
}
