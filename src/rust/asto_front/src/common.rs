use super::*;

#[wasm_bindgen]
extern "C" {
    pub fn login();
    pub fn logout();
    pub fn loaded();
}

pub fn main_js_helper() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let href = window().unwrap_throw().location().href().unwrap_throw();
    *OP_MODE.write().unwrap() = OpMode::from_href(&href);
    dominator::append_dom(&dominator::get_id("app"), render::app());
    init_web_socket();
    Ok(())
}

lazy_static::lazy_static! {
    pub static ref OP_MODE: std::sync::RwLock<OpMode> = std::sync::RwLock::new(OpMode::Demo);
}

pub static mut WEB_SOCKET: Option<WebSocket> = None;

static mut RETRY_TIMEOUT: Option<Timeout> = None;

enum When {
    NoResponseFromServer,
    Anyway,
}

const RETRY_INIT_WEB_SOCKET_TIMEOUT_SECS: u8 = 5;
fn retry_init_web_socket(when: When) {
    unsafe {
        if matches!(when, When::NoResponseFromServer) {
            cancel_retry_init_web_socket();
        }
        if RETRY_TIMEOUT.is_none() {
            RETRY_TIMEOUT = Some(Timeout::new(
                RETRY_INIT_WEB_SOCKET_TIMEOUT_SECS as u32 * 1000,
                move || {
                    RETRY_TIMEOUT = None;
                    WEB_SOCKET = None;
                    *APP.data.is_alive_ws.lock_mut() = false;
                    warn!(@ "will reset WebSocket connection");
                    init_web_socket();
                },
            ));
        }
    }
}

pub fn cancel_retry_init_web_socket() {
    unsafe {
        if let Some(timeout) = RETRY_TIMEOUT.take() {
            timeout.cancel();
        }
    }
}

static mut PING_TIMEOUT: Option<Timeout> = None;

const PING_TIMEOUT_SECS: u8 = 5;
fn ping(timeout_secs: Option<u8>) {
    unsafe {
        if let Some(timeout) = PING_TIMEOUT.take() {
            timeout.cancel();
        }
        PING_TIMEOUT = Some(Timeout::new(
            timeout_secs.unwrap_or(PING_TIMEOUT_SECS) as u32 * 1_000,
            move || {
                PING_TIMEOUT = None;
                send_client_message(ClientMessage::Ping);
                retry_init_web_socket(When::NoResponseFromServer);
            },
        ));
    }
}

pub fn send_client_message(message: ClientMessage) {
    let encoded: Vec<u8> = message.encoded();
    unsafe {
        if let Some(ws) = &WEB_SOCKET {
            match ws.send_with_u8_array(&encoded) {
                Ok(_) => {
                    if !matches!(message, ClientMessage::Ping) {
                        ping(None);
                        debug!("sent {:?}", ClientMessageDiscriminants::from(&message));
                    }
                }
                Err(err) => {
                    error!("error sending message: {err:?}");
                }
            }
        }
    }
}

pub fn init_web_socket() {
    let host = window().unwrap_throw().location().host().unwrap_throw();
    let back = if let Some(stripped) = PKG_NAME.strip_suffix("front") {
        let mut ret = stripped.to_owned();
        ret.push_str("back");
        ret
    } else {
        error!(@ "unreachable");
        unreachable!();
    };
    let ws_url = OP_MODE.read().unwrap().ws_url(&host, &back);
    let ws = match WebSocket::new(&ws_url) {
        Err(err) => {
            warn!("failed to esteblish WebSocket connection to {ws_url:?}: {err:?}");
            retry_init_web_socket(When::Anyway);
            None
        }
        Ok(ws) => {
            ws.set_binary_type(web_sys::BinaryType::Arraybuffer); // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
            {
                let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                    cancel_retry_init_web_socket();
                    ping(None);
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        match ServerMessage::from_encoded(&js_sys::Uint8Array::new(&abuf).to_vec())
                        {
                            Err(err) => {
                                error!(@ "{err:?}");
                            }
                            Ok(message) => {
                                if !matches!(message, ServerMessage::Pong) {
                                    debug!(
                                        "recieved {:?}",
                                        ServerMessageDiscriminants::from(&message)
                                    );
                                }
                                process_server_message(message);
                            }
                        }
                    } else {
                        warn!(@ "message event, received Unknown: {:?}", e.data());
                    }
                })
                    as Box<dyn FnMut(MessageEvent)>);
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
            }

            {
                let onerror_callback =
                    Closure::wrap(Box::new(clone!(ws_url => move |e: ErrorEvent| {
                        error!(@
                                "WebSocket connection to {ws_url:?}, error event: {:?}",
                                e
                        );
                        unsafe {
                            WEB_SOCKET = None;
                        }
                        retry_init_web_socket(When::Anyway);
                    })) as Box<dyn FnMut(ErrorEvent)>);
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();
            }

            {
                let onopen_callback = Closure::wrap(clone!(ws_url => Box::new(move |_| {
                    let app = &APP;
                    *app.data.is_alive_ws.lock_mut() = true;
                    debug!("established WebSocket connection to {ws_url:?}");
                    send_client_message(ClientMessage::Version(semver::Version::parse(PKG_VERSION).unwrap()));
                }) as Box<dyn FnMut(JsValue)>));
                ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
                onopen_callback.forget();
            }
            Some(ws)
        }
    };

    unsafe {
        WEB_SOCKET = ws;
    }

    ping(None);
}

pub fn respond_to_pong(_server_message: ServerMessage) {}

pub fn respond_to_version(server_message: ServerMessage) {
    if let ServerMessage::Version(res) = server_message {
        match res {
            Err(err) => error!("{err}"),
            Ok(need_refresh) => {
                if need_refresh {
                    window()
                        .unwrap_throw()
                        .document()
                        .unwrap_throw()
                        .location()
                        .unwrap_throw()
                        .reload()
                        .unwrap_throw();
                } else {
                    let message = App::get_message_init();
                    send_client_message(message);
                }
            }
        }
    } else {
        unreachable!();
    }
}

pub fn respond_to_init_data(server_message: ServerMessage) {
    match server_message.init_data_get().unwrap() {
        Ok(init_data) => {
            App::init(init_data);
        }
        Err(err) => {
            error!("{err}");
        }
    }
}

// ==========================================================

pub static APP: Lazy<Arc<App>> = Lazy::new(App::new);

// pub struct User {
//     pub roles: login_export::AclRoles,
//     pub auth: login_export::AuthRet,
// }

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::new_helper())
    }
    pub fn is_alive_signal() -> impl Signal<Item = bool> {
        APP.data.is_alive_ws.signal().dedupe()
    }
    pub fn get_message_init() -> ClientMessage {
        ClientMessage::init_set(&ClientMessageInit {
            key: Self::init_data_key(),
        })
    }
}

// ==========================================================
