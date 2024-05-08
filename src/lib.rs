mod tetris_engine;
mod tetris_block;
mod shapes;
mod test_utils;
mod square;
mod tee;

#[cfg(test)]
mod tests {
    use crate::shapes::PlayableShape;
    use crate::square::Square;
    use crate::tee::Tee;
    use crate::test_utils::{are_equal, run};
    use crate::tetris_block::TetrisBlock;
    use crate::tetris_engine::TetrisEngine;

    fn tee_generator() -> Box<dyn PlayableShape> {
        Box::new(Tee::new())
    }

    fn square_generator() -> Box<dyn PlayableShape> {
        Box::new(Square::new())
    }

    #[test]
    fn on_first_tick_a_single_square_is_put_into_play() {
        let tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                          ],
        ))
    }

    #[test]
    fn block_can_be_moved_left_when_user_presses_left() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);
        tetris_engine.move_left();

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(3, 19),
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(3, 18),
                              TetrisBlock::new(4, 18),
                          ],
        ))
    }

    #[test]
    fn block_can_not_move_out_of_bounds_on_the_left() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

        run(|| {
            tetris_engine.move_left();
        }, 100);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(0, 19),
                              TetrisBlock::new(1, 19),
                              TetrisBlock::new(0, 18),
                              TetrisBlock::new(1, 18),
                          ],
        ))
    }

    #[test]
    fn block_can_be_moved_right_when_user_presses_right() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);
        tetris_engine.move_right();

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(6, 19),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(6, 18),
                          ],
        ))
    }

    #[test]
    fn block_can_not_move_out_of_bounds_on_the_right() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

        run(|| {
            tetris_engine.move_right();
        }, 100);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(8, 19),
                              TetrisBlock::new(9, 19),
                              TetrisBlock::new(8, 18),
                              TetrisBlock::new(9, 18),
                          ],
        ))
    }

    #[test]
    fn current_shape_can_not_overlap_existing_blocks_when_moving_right() {
        let mut tetris_engine = TetrisEngine::with_initial_state(
            vec![
                TetrisBlock::new(8, 19),
                TetrisBlock::new(9, 19),
                TetrisBlock::new(8, 18),
                TetrisBlock::new(9, 18),
            ],
            square_generator,
        );

        run(|| {
            tetris_engine.move_right();
        }, 100);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(6, 19),
                              TetrisBlock::new(7, 19),
                              TetrisBlock::new(6, 18),
                              TetrisBlock::new(7, 18),
                              TetrisBlock::new(8, 19),
                              TetrisBlock::new(9, 19),
                              TetrisBlock::new(8, 18),
                              TetrisBlock::new(9, 18),
                          ],
        ))
    }

    #[test]
    fn current_shape_can_not_overlap_existing_blocks_when_moving_left() {
        let mut tetris_engine = TetrisEngine::with_initial_state(
            vec![
                TetrisBlock::new(0, 19),
                TetrisBlock::new(1, 19),
                TetrisBlock::new(0, 18),
                TetrisBlock::new(1, 18),
            ],
            square_generator,
        );

        run(|| {
            tetris_engine.move_left();
        }, 100);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(2, 19),
                              TetrisBlock::new(3, 19),
                              TetrisBlock::new(2, 18),
                              TetrisBlock::new(3, 18),
                              TetrisBlock::new(0, 19),
                              TetrisBlock::new(1, 19),
                              TetrisBlock::new(0, 18),
                              TetrisBlock::new(1, 18),
                          ],
        ))
    }

    #[test]
    fn block_drops_when_user_presses_down() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);
        tetris_engine.drop();

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(4, 1),
                              TetrisBlock::new(5, 1),
                              TetrisBlock::new(4, 0),
                              TetrisBlock::new(5, 0),
                          ],
        ));
    }

    #[test]
    fn incrementing_tick_moves_the_current_square_down() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

        tetris_engine.tick();

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(4, 17),
                              TetrisBlock::new(5, 17),
                          ],
        ))
    }

    #[test]
    fn first_block_stops_when_hitting_bottom() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

        run(|| { tetris_engine.tick() }, 19);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(4, 1),
                              TetrisBlock::new(5, 1),
                              TetrisBlock::new(4, 0),
                              TetrisBlock::new(5, 0),
                          ],
        ));
    }

    #[test]
    fn blocks_collide_and_stack() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

        run(|| { tetris_engine.tick() }, 36);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
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
                          ],
        ));
    }

    #[test]
    fn removes_completed_rows_once_current_shape_is_finished() {
        let mut tetris_engine = TetrisEngine::with_initial_state(
            vec![
                TetrisBlock::new(0, 0),
                TetrisBlock::new(1, 0),
                TetrisBlock::new(2, 0),
                TetrisBlock::new(3, 0),
                TetrisBlock::new(6, 0),
                TetrisBlock::new(7, 0),
                TetrisBlock::new(8, 0),
                TetrisBlock::new(9, 0),
            ],
            square_generator,
        );

        run(|| { tetris_engine.tick() }, 19);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(4, 0),
                              TetrisBlock::new(5, 0),
                          ],
        ));
    }

    #[test]
    fn drops_blocks_above_removed_row() {
        let mut tetris_engine = TetrisEngine::with_initial_state(
            vec![
                TetrisBlock::new(0, 1),
                TetrisBlock::new(1, 1),
                TetrisBlock::new(2, 1),
                TetrisBlock::new(3, 1),
                TetrisBlock::new(6, 1),
                TetrisBlock::new(7, 1),
                TetrisBlock::new(8, 1),
                TetrisBlock::new(9, 1),
            ],
            square_generator,
        );

        run(|| { tetris_engine.tick() }, 19);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 19),
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(4, 0),
                              TetrisBlock::new(5, 0),
                          ],
        ));
    }

    #[test]
    fn on_first_tick_a_single_tee_is_put_into_play() {
        let tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(5, 19),
                              TetrisBlock::new(4, 18),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(6, 18),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_be_rotated_90_degrees() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();

        tetris_engine.rotate();

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(6, 17),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(5, 17),
                              TetrisBlock::new(5, 16),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_be_rotated_180_degrees() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();

        run(||{tetris_engine.rotate()},2);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(5, 16),
                              TetrisBlock::new(6, 17),
                              TetrisBlock::new(5, 17),
                              TetrisBlock::new(4, 17),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_be_rotated_270_degrees() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();

        run(||{tetris_engine.rotate()},3);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(4, 17),
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(5, 17),
                              TetrisBlock::new(5, 16),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_be_rotated_360_degrees() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();

        run(||{tetris_engine.rotate()},4);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(5, 18),
                              TetrisBlock::new(4, 17),
                              TetrisBlock::new(5, 17),
                              TetrisBlock::new(6, 17),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_not_be_rotated_out_of_bounds_on_the_right() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();
        run(||{tetris_engine.rotate()},3);
        run(||{tetris_engine.move_right()},4);
        run(||{tetris_engine.rotate()},3);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(8, 17),
                              TetrisBlock::new(9, 18),
                              TetrisBlock::new(9, 17),
                              TetrisBlock::new(9, 16),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_not_be_rotated_out_of_bounds_on_the_left() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();
        run(||{tetris_engine.rotate()},1);
        run(||{tetris_engine.move_left()},5);
        run(||{tetris_engine.rotate()},1);


        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(1, 17),
                              TetrisBlock::new(0, 18),
                              TetrisBlock::new(0, 17),
                              TetrisBlock::new(0, 16),
                          ],
        ))
    }

    #[test]
    fn a_tee_can_be_rotated_near_the_border_on_the_right() {
        let mut tetris_engine = TetrisEngine::with_initial_state(vec![], tee_generator);

        tetris_engine.tick();
        run(||{tetris_engine.rotate()},2);
        run(||{tetris_engine.move_right()},5);
        run(||{tetris_engine.rotate()},1);

        assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                          &vec![
                              TetrisBlock::new(7, 17),
                              TetrisBlock::new(8, 18),
                              TetrisBlock::new(8, 17),
                              TetrisBlock::new(8, 16),
                          ]
        ))
    }
}