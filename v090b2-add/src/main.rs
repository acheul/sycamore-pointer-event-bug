use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_console::log;

fn main() {
  sycamore::render(App);
}

#[component]
fn App<G: Html>() -> View<G> {

  // attach pointerdown event handler inside of on_mount scope to (3);
  let node_ref = create_node_ref();
  
  on_mount(move || {
    let node: web_sys::EventTarget = node_ref.get::<DomNode>().unchecked_into();
    let cb: Closure<dyn FnMut(web_sys::PointerEvent)> = Closure::<dyn FnMut(_)>::new(move |_: web_sys::PointerEvent| {
      log!("pointerdown(3)");
    });

    node.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref()).unwrap_throw();

    on_cleanup(move || {
      node.remove_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref()).unwrap_throw();
    })
  });


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

      // (2) pointerdown attached
      // => working when part (3) is enabled; not working without part (3) in choromium browsers;
      div(id="id2", style="width: 200px; height: 200px; border: 1px solid black",
        on:pointerdown = move |_| {
          log!("pointerdown(2)");
        }
      ) {
        "pointerdown"
      }

      // (3) pointerdown attached explicitly inside of on_mount scope
      // => working
      div(ref=node_ref, id="id3", style="width: 200px; height: 200px; border: 1px solid black") {
        "pointerdown"
      }
    }
  }
}