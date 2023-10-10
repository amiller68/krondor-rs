use krondor::app::App;

#[cfg(not(target_arch = "wasm32"))]
use krondor::error::KrondorResult;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> KrondorResult<()> {
    App::new().run()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    App::new().run();
}
