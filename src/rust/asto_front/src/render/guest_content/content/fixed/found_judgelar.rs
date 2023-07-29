use super::*;

pub fn render() -> impl Signal<Item = Option<Dom>> {
    App::filtered_judgelar_signal().map(|vec| {
        if vec.is_empty() {
            None
        } else {
            let count = vec.len();
            Some(html!("div", {
                .class("found")
                .text(&common_macros2::plural!(count,
                    1 format!("Найден {count} судья"),
                    2 format!("Найдены {count} судьи"),
                    5 format!("Найдены {count} судьей"),

                ))
            }))
        }
    })
}
