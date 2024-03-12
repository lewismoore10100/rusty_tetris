use crate::tetris_block::TetrisBlock;

pub fn run<F: FnMut() -> ()>(mut f: F, times: u32){
    for _n in 0..times{
        f();
    }
}

pub fn are_equal(actual: &Vec<&TetrisBlock>, expected: &Vec<TetrisBlock>) -> bool{
    let is_equal = actual.iter().zip(expected.iter()).filter(|&(a, b)| !a.same_position(b)).count() == 0;

    if !is_equal {
        eprintln!("{}", "Actual:");
        serde_json::to_writer_pretty(std::io::stderr(), actual).unwrap();
        eprintln!("{}", "Expected:");
        serde_json::to_writer_pretty(std::io::stderr(), expected).unwrap();
        false
    }
    else {
        true
    }
}