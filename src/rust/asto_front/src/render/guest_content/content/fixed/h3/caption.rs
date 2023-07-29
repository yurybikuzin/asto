use super::*;

pub fn render() -> Dom {
    html!("span", {
        .class("caption")
        .text_signal(
            APP.data.route.signal_cloned().map(|route| match route {
                Route::Guest(GuestRoute{ kind: GuestRouteKind::Dancer{..}, ..}) => "Поиск танцора",
                Route::Guest(GuestRoute{ kind: GuestRouteKind::Judge{..}, ..}) => "Поиск судьи",
                _ => unreachable!(),
            })
        )
    })
}
