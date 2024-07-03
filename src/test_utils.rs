use crate::tetris_block::TetrisBlock;

pub fn run<F: FnMut() -> ()>(mut f: F, times: u32) {
    for _n in 0..times {
        f();
    }
}

pub fn are_equal(actual: &Vec<TetrisBlock>, expected: &Vec<TetrisBlock>) -> bool {
    let contains_same_items = actual.iter().zip(expected.iter()).filter(|&(a, b)| !a.same_position(b)).count() == 0;
    let are_same_length = actual.len() == expected.len();

    if !contains_same_items || !are_same_length {
        eprintln!("|{}|", "Actual");
        actual.iter().for_each(|b| {
            eprintln!("| x:{} y:{} id:{} |", b.x, b.y, b.id);
        });
        eprintln!("|{}|", "Expected");
        expected.iter().for_each(|b| {
            eprintln!("| x:{} y:{} id:{} |", b.x, b.y, b.id);
        });
        false
    } else {
        true
    }
}