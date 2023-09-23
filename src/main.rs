
use krondor::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Hello, world!");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    leptos::mount_to_body(|cx| leptos::view! { cx, <App/> })
}
