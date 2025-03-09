use wasm_bindgen::prelude::*;
use leptos::*;
use leptos::mount::mount_to_body;

mod app;
mod components;
mod error_template;
mod pages;

#[wasm_bindgen(start)]
pub fn main() {
    use log::Level;
    console_log::init_with_level(Level::Debug).expect("Failed to initialize logger");
    console_error_panic_hook::set_once();

    mount_to_body(app::App);
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
