use leptos::*;
use crate::tetris_engine::TetrisEngine;

#[component]
pub fn TetrisGrid() -> impl IntoView {
    let (engine , set_engine) = create_signal(TetrisEngine::new());


    view! {
        <div id="render_grid">
            {
                let blocks = engine.with(|e| e.blocks_for_rendering());
                (0..20).rev().map(|y| {
                    (0..10).map(|x| view!{
                        <div>
                        { if blocks.iter().any(|b| b.x == x && b.y == y)
                            { "X" } else { "" }
                        }
                        </div>
                    }).collect_view()
                }).collect_view()
            }
        </div>
    }
}