use leptos::*;
use crate::tetris_grid::TetrisGrid;

mod tetris_grid;
mod game_engine_setup;
mod tetris_block;
mod render_blocks;
mod tetris_engine;
mod shapes;
mod rotation_position;
mod direction;
mod scoring;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <TetrisGrid/> })
}