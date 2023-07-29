use super::*;

pub fn render() -> impl Signal<Item = Option<Dom>> {
    App::filtered_dancerlar_signal().map(|vec| {
        if vec.is_empty() {
            None
        } else {
            let count = vec.len();
            Some(html!("div", {
                .class("found")
                .text(&common_macros2::plural!(count,
                    1 format!("Найден {count} танцор"),
                    2 format!("Найдены {count} танцора"),
                    5 format!("Найдены {count} танцоров"),
                ))
            }))
        }
    })
}
