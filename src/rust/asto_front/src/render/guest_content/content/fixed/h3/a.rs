use super::*;

pub fn render() -> Dom {
    html!("a", {
        .attr_signal("href",
            map_ref! {
                let route = APP.data.route.signal_cloned(),
                let search = APP.data.public_search.signal_cloned()
            => {
                match route {
                    Route::Guest(GuestRoute { kind, ..  }) => Route::Guest(GuestRoute{
                        kind:
                            match kind {
                                GuestRouteKind::Dancer{sort_by, ..} => GuestRouteKind::Judge(
                                    match sort_by {
                                        DancerSortBy::Name => JudgeSortBy::Name,
                                        DancerSortBy::ExternalId => JudgeSortBy::ExternalId,
                                        _ => JudgeSortBy::iter().next().unwrap(),
                                    }
                                ),
                                GuestRouteKind::Judge(sort_by) => GuestRouteKind::Dancer{
                                    sort_by: match sort_by {
                                        JudgeSortBy::Name => DancerSortBy::Name,
                                        JudgeSortBy::ExternalId => DancerSortBy::ExternalId,
                                        _ => DancerSortBy::iter().next().unwrap(),
                                    },
                                    expanded: std::collections::HashSet::new(),
                                },
                            },
                        did_press: false,
                        search: search.clone(),
                    } ),
                    _ => unreachable!(),
                }.to_url()
            }}
        )
        .text_signal(
            APP.data.route.signal_cloned().map(|route| match route {
                Route::Guest(GuestRoute{ kind: GuestRouteKind::Dancer{..}, ..}) => "судьи",
                Route::Guest(GuestRoute{ kind: GuestRouteKind::Judge(_) ,..}) => "танцоры",
                _ => unreachable!(),
            })
        )
        .with_node!(_element => {
            .event(move |_event: events::Click| {
                cancel_delayed_go_to_url();
            })
        })
    })
}
