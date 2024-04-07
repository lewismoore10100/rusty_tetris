mod tetris_engine;
mod tetris_block;
mod square;
mod test_utils;

#[cfg(test)]
mod tests {
    use crate::test_utils::{are_equal, run};
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
    fn block_can_be_moved_left_when_user_presses_left() {
        let mut tetris_engine = TetrisEngine::new();
        tetris_engine.move_left();

        assert!(are_equal(&tetris_engine.generate_blocks(),
                          &vec![
                              TetrisBlock::new(3, 19),
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(3, 18),
                              TetrisBlock::new(4, 18)
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

    #[test]
    fn blocks_collide_and_stack() {
        let mut tetris_engine = TetrisEngine::new();

        run(||{tetris_engine.tick()}, 36);

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
                      TetrisBlock::new(4, 3),
                      TetrisBlock::new(5, 3),
                      TetrisBlock::new(4, 2),
                      TetrisBlock::new(5, 2),

                  ]
        ));
    }
}