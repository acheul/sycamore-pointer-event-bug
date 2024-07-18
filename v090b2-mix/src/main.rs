use sycamore::prelude::*;
use gloo_console::log;


fn main() {
  sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {

  use sycamore::builder::prelude::*;
  main()
    .on(ev::pointerdown, |_| {
      log!("pointerdown1")
    })
    .c(Sub())
    .view()
}

#[component]
pub fn Sub<G: Html>() -> View<G> {

  // Hereby pointer event works too when it's under the Builder syntax component who has pointer event;
  view! {
    div(
      id="id2", style="width: 200px; height: 200px; border: 1px solid black; display: flex; justify-content: space-between;",
      on:pointerup=move |_| { log!("pointerup2"); }
    ) {
      "pointerup2"
    }
  }
}