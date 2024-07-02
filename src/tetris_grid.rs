use leptos::*;

#[component]
pub fn TetrisGrid() -> impl IntoView {
    view! {
        <div id="render_grid">
            {
                (0..200).map(|i| view!{ <div></div> }).collect_view()
            }
        </div>
    }
}