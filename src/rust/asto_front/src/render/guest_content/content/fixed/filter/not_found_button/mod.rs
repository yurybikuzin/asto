use super::*;

mod button;
mod not_found;

pub fn render() -> impl Signal<Item = Option<Dom>> {
    map_ref! {
        let filtered_dancerlar = App::filtered_dancerlar_signal(),
        let filtered_judgelar = App::filtered_judgelar_signal(),
        let filtered_trainerlar = App::filtered_trainerlar_signal(),
        let filtered_clublar = App::filtered_clublar_signal(),
        let did_press = APP.data.did_press.signal()
    =>
        if  !filtered_dancerlar.is_empty()
            || !filtered_judgelar.is_empty()
            || !filtered_trainerlar.is_empty()
            || !filtered_clublar.is_empty()
            {
            None
        } else {
            Some(
                if !*did_press {
                    button::render()
                } else {
                    not_found::render()
                }
            )
        }
    }
}
