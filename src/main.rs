use leptos::*;
use crate::tetris_grid::TetrisGrid;

mod tetris_grid;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <TetrisGrid/> })
}