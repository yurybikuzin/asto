use super::*;

pub fn render() -> Dom {
    html!("div", {
        .class("input_wrapper")
        .children([
            html!("input" => HtmlInputElement, {
                .prop_signal("value", APP.data.public_search.signal_cloned().dedupe_cloned().map(|s| s.unwrap_or_default()))
                .attr("type", "text")
                .attr("id", "input")
                .attr_signal("placeholder", APP.data.route.signal_cloned().map(|route| match route.unwrap_or_else(route_default) {
                Route::Guest(GuestRoute{ kind, ..}) => {
                    match kind {
                        GuestRouteKind::Dancer{..} => "Номер или Фамилия танцора",
                        GuestRouteKind::Judge(_) => "Номер или Фамилия судьи",
                        GuestRouteKind::Trainer(_) => "Фамилия тренера",
                        GuestRouteKind::Club(_) => "Название клуба",
                    }
                }
                _ => unreachable!(),
            }))
                .attr("autocomplete", "off")
                .attr("autocorrect", "off")
                .future(APP.data.suggestlar_signal()
                    .for_each(move |suggestlar| {
                        let ret = if let Some(suggestlar) = suggestlar {
                            if suggestlar.len() == 1 {
                                Some(suggestlar.first().unwrap().to_owned())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        APP.data.suggest_only.set_neq(ret);
                        async {}
                    })
                )
                .with_node!(element => {
                    .event(clone!(element => move |event: events::KeyDown| {
                        match event.key().as_str() {
                            "Enter" => {
                                if let Some(suggest_only) = &*APP.data.suggest_only.lock_ref() {
                                    go_to_url(&route_to_url(&APP.data.suggest_route(Some(suggest_only.to_owned()))));
                                }
                            }
                            "ArrowDown" => {
                                process_input(element.value());
                            }
                            _ => { }
                        }
                    }))
                    .event(move |_event: events::Input| {
                        process_input(element.value());
                        *APP.data.did_press.lock_mut() = false;
                    })
                })
               .future(APP.data.public_search.signal_cloned().dedupe_cloned().for_each(|_change| {
                   delayed_update_url();
                   async {}
               }))
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "button")
                .attr("id", "clear")
                .with_node!(_element => {
                    .event(move |_event: events::Click| {
                        APP.data.suggest_input.replace(None);
                        go_to_url(&route_to_url(&APP.data.suggest_route(None)));
                    })
                })
            })
        ])
    })
}

fn process_input(s: String) {
    APP.data
        .public_search
        .replace(if s.is_empty() { None } else { Some(s.clone()) });
    let s = s.trim();
    if s.chars().count() >= 2
        && !s.starts_with('#')
        && s.split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .count()
            == 1
    {
        let suggest_input = APP.data.suggest_input.lock_ref().clone();
        let need_set = if let Some(suggest_input) = suggest_input {
            suggest_input != s
        } else {
            true
        };
        if need_set {
            APP.data.suggest_input.replace(Some(s.to_owned()));
            APP.data.suggest_selected.replace(false);
        }
    } else {
        APP.data.suggest_input.replace(None);
    }
}
