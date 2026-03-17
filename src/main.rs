use leptos::mount::mount_to_body;

mod app;
mod components;
mod features;
mod pages;
mod state;

fn main() {
    mount_to_body(app::App);
}
