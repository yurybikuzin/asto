use super::*;

pub fn render() -> Dom {
    html!("div", {
        .class("not_found")
        .text_signal(
            APP.data.public_search.signal_cloned().map(|search|
               if search.map(|s|s.trim().is_empty()).unwrap_or(true) {
                    "Вы ничего не ввели"
               } else {
                    "По Вашему запросу ничего не найдено"
               }
            )
        )
    })
}
