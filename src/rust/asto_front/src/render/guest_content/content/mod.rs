use super::*;

mod filtered_clublar_table;
mod filtered_dancerlar_table;
mod filtered_judgelar_table;
mod filtered_trainerlar_table;
mod fixed;

pub use filtered_dancerlar_table::DancerScorePeriod;

pub fn render() -> Dom {
    html!("div", { .class("content")
        .future(APP.data.dancerlar_map.signal_map_cloned().for_each(|_change| {
            update_dancerlar();
            async {}
        }))
        .future(APP.data.judgelar_map.signal_map_cloned().for_each(|_change| {
            update_judgelar();
            async {}
        }))
        .future(APP.data.trainerlar_map.signal_map_cloned().for_each(|_change| {
            update_trainerlar();
            async {}
        }))
        .future(APP.data.clublar_map.signal_map_cloned().for_each(|_change| {
            update_clublar();
            async {}
        }))
        .child(fixed::render())
        .child_signal(map_ref!{
            let rows = App::filtered_dancerlar_signal()
            , let route = APP.data.route.signal_cloned()
        => {
           (!rows.is_empty()).then(|| filtered_dancerlar_table::render(rows.clone(), route.clone().unwrap_or_else(route_default)))
        }})
        .child_signal(App::filtered_judgelar_signal().map(|vec| (!vec.is_empty()).then(|| filtered_judgelar_table::render(vec))))
        .child_signal(App::filtered_trainerlar_signal().map(|vec| (!vec.is_empty()).then(|| filtered_trainerlar_table::render(vec))))
        .child_signal(App::filtered_clublar_signal().map(|vec| (!vec.is_empty()).then(|| filtered_clublar_table::render(vec))))
    })
}

fn update_dancerlar() {
    APP.data.dancerlar.replace(
        APP.data
            .dancerlar_map
            .lock_ref()
            .values()
            .cloned()
            .collect::<Vec<_>>(),
    );
}

fn update_judgelar() {
    APP.data.judgelar.replace(
        APP.data
            .judgelar_map
            .lock_ref()
            .values()
            .cloned()
            .collect::<Vec<_>>(),
    );
}

fn update_trainerlar() {
    let personlar_map = &*APP.data.personlar_map.lock_ref();
    let trainerlar = APP
        .data
        .dancerlar_map
        .lock_ref()
        .values()
        .map(|dancer| dancer.trainer)
        .collect::<HashSet<_>>();
    let trainerlar_map = &*APP.data.trainerlar_map.lock_ref();
    APP.data.trainerlar.replace(
        trainerlar
            .into_iter()
            .filter_map(|trainer| {
                trainerlar_map.get(&trainer).and_then(|trainer| {
                    (personlar_map
                        .get(&trainer.person)
                        .map(|person| {
                            !(person.last_name == 1
                                && person.first_name == 1
                                && person.second_name == 1)
                        })
                        .unwrap_or(false))
                    .then_some(trainer)
                })
            })
            .cloned()
            .collect(),
    );
}

fn update_clublar() {
    let clublar = APP
        .data
        .dancerlar_map
        .lock_ref()
        .values()
        .map(|dancer| dancer.club)
        .chain(
            APP.data
                .judgelar_map
                .lock_ref()
                .values()
                .map(|judge| judge.club),
        )
        .chain(
            APP.data
                .trainerlar_map
                .lock_ref()
                .values()
                .map(|trainer| trainer.club),
        )
        .collect::<HashSet<_>>();
    let clublar_map = &*APP.data.clublar_map.lock_ref();
    APP.data.clublar.replace({
        clublar
            .into_iter()
            .filter_map(|club| {
                clublar_map
                    .get(&club)
                    .and_then(|club| (club.value != 1).then_some(club))
            })
            .cloned()
            .collect()
    });
}
