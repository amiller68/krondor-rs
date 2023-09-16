use leptos::*;
use common::hello;

fn main() {
    let msg = hello();
    mount_to_body(|cx| view! { cx, <p> { msg } </p> })
}