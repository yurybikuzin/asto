use super::*;

// mod a;
// mod caption;

pub fn render() -> Dom {
    html!("h3", {
        .children([
            html!("label", {
                .attr("for", "search")
                .text("Поиск")
            }),
            html!("select", {
                .attr("id", "search")
                .children(GuestRouteKindDiscriminants::iter().map(|i|
                    html!("option", {
                        .attr("value", &(i as u8).to_string())
                        .attr_signal("selected",
                            APP.data.route.signal_cloned().map(move |route|
                                if let Route::Guest(GuestRoute{
                                    kind,
                                    ..
                                }) = route.unwrap_or_else(route_default) {
                                    (i == GuestRouteKindDiscriminants::from(&kind)).then_some("")
                                } else {
                                    None
                                }
                            )
                        )
                        .text(&i.to_string())
                    })
                ))
                .with_node!(element => {
                    .event(move |_event: events::Change| {
                        // todo!();
                        if let Some(element) = element.dyn_ref::<HtmlSelectElement>() {
                            let value = element.value().parse::<usize>().unwrap();
                            if let Some(guest_route_kind_discriminants) = GuestRouteKindDiscriminants::from_repr(value) {
                                let route = &*APP.data.route.lock_ref();
                                let search = &*APP.data.public_search.lock_ref();
                                let url = route_to_url(&match route.clone().unwrap_or_else(route_default) {
                                    Route::Guest(GuestRoute { kind, ..  }) => Route::Guest(GuestRoute{
                                        kind:
                                            match kind {
                                                GuestRouteKind::Dancer{sort_by, ..} =>
                                                    match guest_route_kind_discriminants {
                                                        GuestRouteKindDiscriminants::Dancer => GuestRouteKind::Dancer{ sort_by, expanded: std::collections::HashSet::new() },
                                                        GuestRouteKindDiscriminants::Judge => GuestRouteKind::Judge(
                                                            match sort_by {
                                                                DancerSortBy::Name => JudgeSortBy::Name,
                                                                DancerSortBy::ExternalId => JudgeSortBy::ExternalId,
                                                                _ => JudgeSortBy::iter().next().unwrap(),
                                                            }
                                                        ),
                                                        GuestRouteKindDiscriminants::Trainer => GuestRouteKind::Trainer(TrainerSortBy::iter().next().unwrap()),
                                                        GuestRouteKindDiscriminants::Club => GuestRouteKind::Club(ClubSortBy::iter().next().unwrap()),
                                                    },
                                                GuestRouteKind::Judge(sort_by) =>
                                                    match guest_route_kind_discriminants {
                                                        GuestRouteKindDiscriminants::Dancer => GuestRouteKind::Dancer{
                                                            sort_by: match sort_by {
                                                                JudgeSortBy::Name => DancerSortBy::Name,
                                                                JudgeSortBy::ExternalId => DancerSortBy::ExternalId,
                                                                _ => DancerSortBy::iter().next().unwrap(),
                                                            },
                                                            expanded: std::collections::HashSet::new()
                                                        },
                                                        GuestRouteKindDiscriminants::Judge => GuestRouteKind::Judge(sort_by),
                                                        GuestRouteKindDiscriminants::Trainer => GuestRouteKind::Trainer(TrainerSortBy::iter().next().unwrap()),
                                                        GuestRouteKindDiscriminants::Club => GuestRouteKind::Club(ClubSortBy::iter().next().unwrap()),
                                                    },
                                                GuestRouteKind::Trainer(sort_by) =>
                                                    match guest_route_kind_discriminants {
                                                        GuestRouteKindDiscriminants::Dancer => GuestRouteKind::Dancer{
                                                            sort_by: DancerSortBy::iter().next().unwrap(),
                                                            expanded: std::collections::HashSet::new()
                                                        },
                                                        GuestRouteKindDiscriminants::Judge => GuestRouteKind::Judge(JudgeSortBy::iter().next().unwrap()),
                                                        GuestRouteKindDiscriminants::Trainer => GuestRouteKind::Trainer(sort_by),
                                                        GuestRouteKindDiscriminants::Club => GuestRouteKind::Club(ClubSortBy::iter().next().unwrap()),
                                                    },
                                                GuestRouteKind::Club(sort_by) =>
                                                    match guest_route_kind_discriminants {
                                                        GuestRouteKindDiscriminants::Dancer => GuestRouteKind::Dancer{
                                                            sort_by: DancerSortBy::iter().next().unwrap(),
                                                            expanded: std::collections::HashSet::new()
                                                        },
                                                        GuestRouteKindDiscriminants::Judge => GuestRouteKind::Judge(JudgeSortBy::iter().next().unwrap()),
                                                        GuestRouteKindDiscriminants::Trainer => GuestRouteKind::Trainer(TrainerSortBy::iter().next().unwrap()),
                                                        GuestRouteKindDiscriminants::Club => GuestRouteKind::Club(sort_by),
                                                    },
                                            },
                                        did_press: false,
                                        search: search.clone(),
                                    } ),
                                    _ => unreachable!(),
                                });

                                cancel_delayed_go_to_url();
                                go_to_url(&url);
                            }
                        }
                    })
                })
            }),
        ])
        // .children([
        //     caption::render(),
        //     a::render(),
        // ])
    })
}
