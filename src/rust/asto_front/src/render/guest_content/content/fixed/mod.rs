use super::*;

mod filter;
mod found_clublar;
mod found_dancerlar;
mod found_judgelar;
mod found_trainerlar;
mod h3;
mod sort_clublar;
mod sort_dancerlar;
mod sort_judgelar;
mod sort_trainerlar;
mod suggest;

pub fn render() -> Dom {
    html!("div", { .class("fixed")
        .future(APP.data.did_press.signal().dedupe().for_each(|_change| {
            delayed_update_url();
            async {}
        }))
        .future(APP.data.public_search.signal_cloned().dedupe_cloned().for_each(|_change| {
            delayed_update_url();
            async {}
        }))
        .children([
            html!("h3", {
                .class("protocols")
                .child(html!("a", {
                    .attr("href", &format!("{}/asto_back/beta", OP_MODE.read().unwrap().route_prefix()))
                    .text("Протоколы фестивалей")
                }))
            }),
            html!("h2", {
                .text("База данных")
            }),
            h3::render(),
            filter::render(),
        ])
        .child_signal(suggest::render_signal())

        .child_signal(found_dancerlar::render())
        .child_signal(sort_dancerlar::render())

        .child_signal(found_judgelar::render())
        .child_signal(sort_judgelar::render())

        .child_signal(found_trainerlar::render())
        .child_signal(sort_trainerlar::render())

        .child_signal(found_clublar::render())
        .child_signal(sort_clublar::render())
    })
}
