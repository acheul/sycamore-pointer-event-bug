use sycamore::{prelude::*, web::html::{ev, EventHandler}};
use wasm_bindgen::prelude::*;
use gloo_console::log;


fn main() {
  sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {

  // Builder syntax's pointer event works well, unlink in view! syntax.
  use sycamore::builder::prelude::*;

  div()
    .attr("id", "id1")
    .attr("style", "width: 500px; height: 500px; border: 1px solid black; display: flex; justify-content: space-between;")
    .on(ev::pointerdown, |_| {
      log!("pointerdown")
    })
    .t("pointerdown")
    .c(Sub())
    .view()
}

#[component]
pub fn Sub<G: Html>() -> View<G> {

  // Hereby pointer event works too when it's under the Builder syntax component.
  view! {
    div(
      id="id2", style="width: 200px; height: 200px; border: 1px solid black; display: flex; justify-content: space-between;",
      on:pointerdown=move |_| { log!("pointerdown2"); }
    ) {
      "pointerdown2"
    }
  }
}