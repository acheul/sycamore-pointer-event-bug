use sycamore::prelude::*;
use gloo_console::log;

fn main() {
  sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {

  let x = create_signal(0i32);

  view! {
    main(style="user-select: none;") {

      div() {
        (x.get())
      }

      // (1) mousedown attached
      div(id="id1", style="width: 200px; height: 200px; border: 1px solid black",
        on:mousedown = move |_| {
          log!("mousedown");
          x.set(x.get()+1);
        }
      ) {
        "mousedown"
      }

      // (2) pointerdown attached => not working in chromium browsers; working in firefox
      div(id="id2", style="width: 200px; height: 200px; border: 1px solid black",
        on:pointerdown = move |_| {
          log!("pointerdown");
          x.set(x.get()+1);
        }
      ) {
        "pointerdown"
      }
    }
  }
}