use sycamore::prelude::*;
use gloo_console::log;

fn main() {
  sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {

  view! {
    main(style="user-select: none;") {

      // (1) mousedown attached
      div(id="id1", style="width: 200px; height: 200px; border: 1px solid black",
        on:mousedown = move |_| {
          log!("mousedown");
        }
      ) {
        "mousedown"
      }

      // (2) pointerdown attached => not working in chromium browsers; working in firefox
      div(id="id2", style="width: 200px; height: 200px; border: 1px solid black",
        on:pointerdown = move |_| {
          log!("pointerdown");
        }
      ) {
        "pointerdown"
      }
    }
  }
}