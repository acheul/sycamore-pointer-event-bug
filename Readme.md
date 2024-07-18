# Sycamore-v0.9.0's PointerEvent error in Chrome (Windows)

**Describe the bug**

PointerEvent is not fired in Chomium browsers (Chrome, Edge, Brave) when building it with Sycamore version 0.9.0-beta2's `view!` syntax.

For example, below code successfully builds a `div` element, however, the pointerdown event does not get fired in Chromium browsers (checked Chrome, Edge, and Brave at Windows). Tricky thing is that the pointerdown event listener is indeed attached to the element when looking up the browsers' devtools panel. It just not get fired. Other events like mousedown works very well (didn't check all the event types though.);

```rust
view! {
  div(on:pointerdown = move |_| { /*do something*/ }) { "pointerdown" } 
}
```

For the comparison, these cases works well:
* Browser-side:
  * using Firefox
* Code-side:
  * Explicitly add a event handler inside of `on_mount` scope
  * using Builder syntax, not `view!` syntax
  * using Sycamore version 0.8.2

<br>

**To Reproduce**

Check crates under this repository:

* `v090b2-on`:
  * using view! syntax, Sycamore=v0.9.0.beta-2;
  * ❎The pointerdown event not gets fired in Chromium browsers.
* `v090b2-add`:
  * Sycamore=v0.9.0.beta-2; added event handlers inside of `on_mount` scope
  * ✅ works well
* `v090b2-builder`:
  * Sycamore=v0.9.0.beta-2, using builder syntax instead of view! syntax
  * ✅ works well
* `v082-on`:
  * Sycamore=v0.8.2, view! syntax
  * ✅ works well

<br>

**Expected behavior**

PointerEvent's handlers get fired as expected


<br>

**Environment**

- Sycamore: 0.9.2-beta.2
- Browser:
  - Chrome: 126.0.6478.183 (latest at the moment)
  - Firefox: 128.0 (latest at the moment)
- OS: Windows

<br>

**Additional context**