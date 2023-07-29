use super::*;

mod guest_content;
pub use guest_content::*;

const RENDER_TIMEOUT_MILLIS: u32 = 250;
static mut RENDER_TIMEOUT: Option<Timeout> = None;

pub fn app() -> Dom {
    unsafe {
        if let Some(timeout) = RENDER_TIMEOUT.take() {
            timeout.cancel();
        }
        if RENDER_TIMEOUT.is_none() {
            RENDER_TIMEOUT = Some(Timeout::new(RENDER_TIMEOUT_MILLIS, move || {
                RENDER_TIMEOUT = None;
            }));
        }
    }

    html!("div", {
        .future(routing::url()
            .signal_ref(|url| route_from_url(url))
            .dedupe_cloned()
            .for_each(move |route| {
                APP.data.route.set_neq(Some(route.clone()));
                if let Route::Guest(GuestRoute{ did_press, search, ..}) = &route {
                    APP.data.did_press.set_neq(*did_press);
                    APP.data.public_search.set_neq(search.clone());
                }
                async {}
            })
        )
        .class_signal("is_alive", App::is_alive_signal())
        .class_signal("is_in_commit", APP.data.is_in_commit.signal())
        .child(guest_content())
    })
}
