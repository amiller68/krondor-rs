use krondor::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    App::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    App::run();
}
