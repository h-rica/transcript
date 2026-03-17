use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod app;
mod components;
mod pages;
mod state;

fn main() {
    mount_to_body(app::App);
}
