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
                    TetrisBlock{x: 4, y: 19},
                    TetrisBlock{x: 5, y: 19},
                    TetrisBlock{x: 4, y: 18},
                    TetrisBlock{x: 5, y: 18},
            ]
        ))

    }

    #[test]
    fn incrementing_tick_moves_the_current_square_down() {
        let mut tetris_engine = TetrisEngine::new();

        tetris_engine.tick();

        assert!(are_equal(&tetris_engine.generate_blocks(),
                          &vec![
                              TetrisBlock{x: 4, y: 18},
                              TetrisBlock{x: 5, y: 18},
                              TetrisBlock{x: 4, y: 17},
                              TetrisBlock{x: 5, y: 17},
                          ]
        ))

    }


    fn are_equal(actual: &Vec<TetrisBlock>, expected: &Vec<TetrisBlock>) -> bool{
        actual == expected
    }
}