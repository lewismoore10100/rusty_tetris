use std::time::Duration;
use blue_engine::StringBufferTrait;
use leptos::*;
use crate::tetris_block::COLOR;
use crate::tetris_engine::TetrisEngine;

#[component]
pub fn TetrisGame() -> impl IntoView {
    let (engine , set_engine) = create_signal(TetrisEngine::new());

    set_interval(move || {
        set_engine.update(move |engine| { engine.tick()});
    }, Duration::from_secs(1));


    view! {
        <div id="render_grid">
            {move || {
                let blocks = engine.with(|e| e.blocks_for_rendering());
                (0..20).rev().map(|y| {
                    (0..10).map(|x| {
                        blocks.iter()
                            .find(|b| b.x == x && b.y == y)
                            .map(|b| {
                                let hex_color = color_as_hex(&b.color);
                                view! {
                                    <div style=("background-color", move || format!("{}", hex_color))></div>
                                }
                            })
                            .unwrap_or_else(|| { view! { <div></div> }})
                    }).collect_view()
                }).collect_view()
            }}
        </div>
    }
}

fn color_as_hex(color: &COLOR) -> String {
    match color {
        COLOR::BLUE => {"#0000FF".as_string()}
        COLOR::GREEN => {"#008000".as_string()}
        COLOR::YELLOW => {"#FFFF00".as_string()}
        COLOR::RED => {"#FF0000".as_string()}
        COLOR::ORANGE => {"#FFA500".as_string()}
        COLOR::TURQUOISE => {"#40E0D0".as_string()}
    }
}