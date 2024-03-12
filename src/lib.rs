#[macro_use]
mod browser;
mod engine;
mod game;

use engine::GameLoop;
use game::GameObj;
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    /*
    wasm_bindgen_futures::spawn_local(async move {
        let game = GameObj::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });
    */
    browser::spawn_local(async move {
        let game = GameObj::new();

        GameLoop::start(game)
            //.await
            .expect("Could not start game loop");
    });
    
    Ok(())
}
