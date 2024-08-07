use crate::shapes::i::I;
use crate::shapes::l::L;
use crate::shapes::PlayableShape;
use crate::shapes::s::S;
use crate::shapes::square::Square;
use crate::shapes::t::T;
use crate::shapes::z::Z;
use crate::test_utils::{are_equal, run};
use crate::tetris_block::TetrisBlock;
use crate::tetris_engine::TetrisEngine;

fn i_generator() -> Box<dyn PlayableShape> {
    Box::new(I::new())
}

fn z_generator() -> Box<dyn PlayableShape> {
    Box::new(Z::new())
}

fn s_generator() -> Box<dyn PlayableShape> {
    Box::new(S::new())
}

fn l_generator() -> Box<dyn PlayableShape> {
    Box::new(L::new())
}

fn t_generator() -> Box<dyn PlayableShape> {
    Box::new(T::new())
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
fn block_drops_when_user_presses_drop() {
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
fn block_moves_down_when_user_presses_down() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], square_generator);

    tetris_engine.move_down();

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(5, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(5, 17),
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
fn blocks_collide_and_stacks() {
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
    let tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

    tetris_engine.tick();

    run(|| { tetris_engine.rotate() }, 2);


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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

    tetris_engine.tick();

    run(|| { tetris_engine.rotate() }, 3);


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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

    tetris_engine.tick();

    run(|| { tetris_engine.rotate() }, 4);


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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 3);
    run(|| { tetris_engine.move_right() }, 4);
    run(|| { tetris_engine.rotate() }, 3);


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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 1);
    run(|| { tetris_engine.move_left() }, 5);
    run(|| { tetris_engine.rotate() }, 1);


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
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], t_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 2);
    run(|| { tetris_engine.move_right() }, 5);
    run(|| { tetris_engine.rotate() }, 1);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(7, 17),
                          TetrisBlock::new(8, 18),
                          TetrisBlock::new(8, 17),
                          TetrisBlock::new(8, 16),
                      ],
    ))
}

#[test]
fn on_first_tick_a_single_l_is_put_into_play() {
    let tetris_engine = TetrisEngine::with_initial_state(vec![], l_generator);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 19),
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(5, 17),
                      ],
    ))
}

#[test]
fn a_l_can_be_rotated_90_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], l_generator);

    tetris_engine.tick();
    tetris_engine.rotate();

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(6, 17),
                          TetrisBlock::new(5, 17),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(4, 16),
                      ],
    ))
}

#[test]
fn a_l_can_be_rotated_180_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], l_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 2);


    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(5, 15),
                          TetrisBlock::new(5, 16),
                          TetrisBlock::new(5, 17),
                          TetrisBlock::new(4, 17),
                      ],
    ))
}

#[test]
fn a_l_can_be_rotated_270_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], l_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 3);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(3, 16),
                          TetrisBlock::new(4, 16),
                          TetrisBlock::new(5, 16),
                          TetrisBlock::new(5, 17),
                      ],
    ))
}

#[test]
fn a_l_can_be_rotated_360_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], l_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 4);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(4, 16),
                          TetrisBlock::new(5, 16),
                      ],
    ))
}

#[test]
fn on_first_tick_a_single_s_is_put_into_play() {
    let tetris_engine = TetrisEngine::with_initial_state(vec![], s_generator);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 19),
                          TetrisBlock::new(5, 19),
                          TetrisBlock::new(3, 18),
                          TetrisBlock::new(4, 18),
                      ],
    ))
}

#[test]
fn a_s_can_be_rotated_90_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], s_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 1);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(4, 16),
                          TetrisBlock::new(3, 18),
                          TetrisBlock::new(3, 17),
                      ],
    ))
}

#[test]
fn a_s_can_be_rotated_back_and_forth() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], s_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 3);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(4, 16),
                          TetrisBlock::new(3, 18),
                          TetrisBlock::new(3, 17),
                      ],
    ))
}

#[test]
fn a_s_can_be_rotated_90_degrees_and_back() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], s_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 2);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(5, 18),
                          TetrisBlock::new(3, 17),
                          TetrisBlock::new(4, 17),
                      ],
    ))
}


fn on_first_tick_a_single_z_is_put_into_play() {
    let tetris_engine = TetrisEngine::with_initial_state(vec![], z_generator);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(3, 19),
                          TetrisBlock::new(4, 19),
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(5, 18),
                      ],
    ))
}

fn on_first_tick_a_single_i_is_put_into_play() {
    let tetris_engine = TetrisEngine::with_initial_state(vec![], i_generator);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 19),
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(4, 16),
                      ],
    ))
}

#[test]
fn a_i_can_be_roated_90_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], i_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 1);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(6, 16),
                          TetrisBlock::new(5, 16),
                          TetrisBlock::new(4, 16),
                          TetrisBlock::new(3, 16),
                      ],
    ))
}

#[test]
fn a_i_can_be_rotated_90_degrees_and_back() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], i_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 2);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(4, 16),
                          TetrisBlock::new(4, 15),
                      ],
    ))
}

#[test]
fn a_z_can_be_rotated_90_degrees() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], z_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 1);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(3, 17),
                          TetrisBlock::new(3, 16),
                      ],
    ))
}

#[test]
fn a_z_can_be_rotated_90_degrees_and_back() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![], z_generator);

    tetris_engine.tick();
    run(|| { tetris_engine.rotate() }, 2);

    assert!(are_equal(&tetris_engine.blocks_for_rendering(),
                      &vec![
                          TetrisBlock::new(3, 18),
                          TetrisBlock::new(4, 18),
                          TetrisBlock::new(4, 17),
                          TetrisBlock::new(5, 17),
                      ],
    ))
}

#[test]
fn completing_a_single_row_scores_40_points() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![
        TetrisBlock::new(0, 0),
        TetrisBlock::new(1, 0),
        TetrisBlock::new(2, 0),
        TetrisBlock::new(3, 0),
        TetrisBlock::new(4, 0),
        TetrisBlock::new(5, 0),
        TetrisBlock::new(6, 0),
        TetrisBlock::new(7, 0),
        TetrisBlock::new(8, 0),
        TetrisBlock::new(9, 0),
    ], l_generator);

    run(|| { tetris_engine.tick() }, 20);

    assert_eq!(tetris_engine.score(), 40);
}

#[test]
fn completing_a_double_row_scores_100_points() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![
        TetrisBlock::new(0, 0),
        TetrisBlock::new(1, 0),
        TetrisBlock::new(2, 0),
        TetrisBlock::new(3, 0),
        TetrisBlock::new(4, 0),
        TetrisBlock::new(5, 0),
        TetrisBlock::new(6, 0),
        TetrisBlock::new(7, 0),
        TetrisBlock::new(8, 0),
        TetrisBlock::new(9, 0),
        TetrisBlock::new(0, 1),
        TetrisBlock::new(1, 1),
        TetrisBlock::new(2, 1),
        TetrisBlock::new(3, 1),
        TetrisBlock::new(4, 1),
        TetrisBlock::new(5, 1),
        TetrisBlock::new(6, 1),
        TetrisBlock::new(7, 1),
        TetrisBlock::new(8, 1),
        TetrisBlock::new(9, 1),
    ], l_generator);

    run(|| { tetris_engine.tick() }, 20);

    assert_eq!(tetris_engine.score(), 100);
}

#[test]
fn completing_a_triple_row_scores_300_points() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![
        TetrisBlock::new(0, 0),
        TetrisBlock::new(1, 0),
        TetrisBlock::new(2, 0),
        TetrisBlock::new(3, 0),
        TetrisBlock::new(4, 0),
        TetrisBlock::new(5, 0),
        TetrisBlock::new(6, 0),
        TetrisBlock::new(7, 0),
        TetrisBlock::new(8, 0),
        TetrisBlock::new(9, 0),
        TetrisBlock::new(0, 1),
        TetrisBlock::new(1, 1),
        TetrisBlock::new(2, 1),
        TetrisBlock::new(3, 1),
        TetrisBlock::new(4, 1),
        TetrisBlock::new(5, 1),
        TetrisBlock::new(6, 1),
        TetrisBlock::new(7, 1),
        TetrisBlock::new(8, 1),
        TetrisBlock::new(9, 1),
        TetrisBlock::new(0, 2),
        TetrisBlock::new(1, 2),
        TetrisBlock::new(2, 2),
        TetrisBlock::new(3, 2),
        TetrisBlock::new(4, 2),
        TetrisBlock::new(5, 2),
        TetrisBlock::new(6, 2),
        TetrisBlock::new(7, 2),
        TetrisBlock::new(8, 2),
        TetrisBlock::new(9, 2),
    ], l_generator);

    run(|| { tetris_engine.tick() }, 20);

    assert_eq!(tetris_engine.score(), 300);
}

#[test]
fn completing_a_triple_row_scores_1200_points() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![
        TetrisBlock::new(0, 0),
        TetrisBlock::new(1, 0),
        TetrisBlock::new(2, 0),
        TetrisBlock::new(3, 0),
        TetrisBlock::new(4, 0),
        TetrisBlock::new(5, 0),
        TetrisBlock::new(6, 0),
        TetrisBlock::new(7, 0),
        TetrisBlock::new(8, 0),
        TetrisBlock::new(9, 0),
        TetrisBlock::new(0, 1),
        TetrisBlock::new(1, 1),
        TetrisBlock::new(2, 1),
        TetrisBlock::new(3, 1),
        TetrisBlock::new(4, 1),
        TetrisBlock::new(5, 1),
        TetrisBlock::new(6, 1),
        TetrisBlock::new(7, 1),
        TetrisBlock::new(8, 1),
        TetrisBlock::new(9, 1),
        TetrisBlock::new(0, 2),
        TetrisBlock::new(1, 2),
        TetrisBlock::new(2, 2),
        TetrisBlock::new(3, 2),
        TetrisBlock::new(4, 2),
        TetrisBlock::new(5, 2),
        TetrisBlock::new(6, 2),
        TetrisBlock::new(7, 2),
        TetrisBlock::new(8, 2),
        TetrisBlock::new(9, 2),
        TetrisBlock::new(0, 3),
        TetrisBlock::new(1, 3),
        TetrisBlock::new(2, 3),
        TetrisBlock::new(3, 3),
        TetrisBlock::new(4, 3),
        TetrisBlock::new(5, 3),
        TetrisBlock::new(6, 3),
        TetrisBlock::new(7, 3),
        TetrisBlock::new(8, 3),
        TetrisBlock::new(9, 3),
    ], l_generator);

    run(|| { tetris_engine.tick() }, 20);

    assert_eq!(tetris_engine.score(), 1200);
}

#[test]
fn the_game_is_over_when_a_new_shape_overlaps_an_existing_block() {
    let mut tetris_engine = TetrisEngine::with_initial_state(vec![
        TetrisBlock::new(5, 19),
        TetrisBlock::new(6, 19),
        TetrisBlock::new(5, 18),
        TetrisBlock::new(6, 18),
        
    ], square_generator);

    run(|| { tetris_engine.tick() }, 1);

    assert_eq!(tetris_engine.is_game_over(), true);
}