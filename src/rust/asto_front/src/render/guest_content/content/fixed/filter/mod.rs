use super::*;

mod input;
mod not_found_button;

pub fn render() -> Dom {
    html!("div", {
        .class("filter")
        .children([
            input::render(),
        ])
        .child_signal( not_found_button::render())
    })
}
