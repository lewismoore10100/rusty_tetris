mod tetris_engine;
mod tetris_block;
mod square;

#[cfg(test)]
mod tests {
    use crate::tetris_block::TetrisBlock;
    use crate::tetris_engine::TetrisEngine;

    #[test]
    fn on_first_tick_a_single_square_is_put_into_play() {
        let tetris_engine = TetrisEngine::new();

        assert!(are_equal(&tetris_engine.generate_blocks(),
    &vec![
                    TetrisBlock::new(4, 19),
                    TetrisBlock::new(5, 19),
                    TetrisBlock::new(4, 18),
                    TetrisBlock::new(5, 18)
            ]
        ))

    }

    #[test]
    fn incrementing_tick_moves_the_current_square_down() {
        let mut tetris_engine = TetrisEngine::new();

        tetris_engine.tick();

        assert!(are_equal(&tetris_engine.generate_blocks(),
            &vec![
                      TetrisBlock::new(4, 18),
                      TetrisBlock::new(5, 18),
                      TetrisBlock::new(4, 17),
                      TetrisBlock::new(5, 17),
                  ]
        ))
    }

    #[test]
    fn first_block_stops_when_hitting_bottom() {
        let mut tetris_engine = TetrisEngine::new();

        run(||{tetris_engine.tick()}, 19);

        assert!(are_equal(&tetris_engine.generate_blocks(),
                  &vec![
                      TetrisBlock::new(4, 19),
                      TetrisBlock::new(5, 19),
                      TetrisBlock::new(4, 18),
                      TetrisBlock::new(5, 18),
                      TetrisBlock::new(4, 1),
                      TetrisBlock::new(5, 1),
                      TetrisBlock::new(4, 0),
                      TetrisBlock::new(5, 0),
                  ]
        ));
    }

    fn run<F: FnMut() -> ()>(mut f: F, times: u32){
        for _n in 0..times{
            f();
        }
    }

    fn are_equal(actual: &Vec<&TetrisBlock>, expected: &Vec<TetrisBlock>) -> bool{
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
}