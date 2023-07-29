use super::*;

pub fn render() -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "button")
        .attr("value", "Найти")
        .with_node!(_element => {
            .event(move |_event: events::Click| {
                 *APP.data.did_press.lock_mut() = true;
            })
        })
    })
}
