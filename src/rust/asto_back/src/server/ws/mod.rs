use super::*;

#[macro_use]
pub mod common;
pub use common::*;

pub const NEED_CACHE_INIT_DATA: bool = true;
// pub const NO_GUEST: bool = false;
// lazy_static::lazy_static! {
// pub static ref LOCAL_AUTH: Option<login_export::AuthRet> = Some(login_export::AuthRet {
//     contact: login_export::AuthContact::Email("yury.bikuzin@gmail.com".to_owned()),
//     details: login_export::AuthRetDetails {
//         nickname: None,
//         name: None,
//         given_name: None,
//         middle_name: None,
//         family_name: None,
//         birthday: None,
//         phone_number: None,
//         email: None,
//         emails: None,
//         gender: None,
//         picture: None,
//     },
// });
// }

// pub enum Kind {
//     // User(login_export::AuthRet),
// }

async fn process_ws_connection_message(ws_connection_id: WsConnectionId, msg: warp::ws::Message) {
    if msg.as_bytes().is_empty() {
        warn!("{}:{}: msg.as_bytes().is_empty()", file!(), line!());
    } else {
        let mut need_send_init_data_after_all: Option<InitDataKey> = None;
        if let Some((message, send)) = match ClientMessage::from_encoded(msg.as_bytes()) {
            Err(err) => {
                error!("{}:{}: {err}", file!(), line!());
                None
            }
            Ok(client_message) => {
                let client_message_discriminants =
                    ClientMessageDiscriminants::from(&client_message);
                match client_message_discriminants {
                    ClientMessageDiscriminants::Ping => respond_to_ping(),
                    ClientMessageDiscriminants::Version => respond_to_version(client_message),
                    ClientMessageDiscriminants::Init => {
                        respond_to_init(client_message, &mut need_send_init_data_after_all)
                    }
                    // ClientMessageDiscriminants::Login => {
                    //     respond_to_login(client_message, ws_connection_id).await
                    // }
                    ClientMessageDiscriminants::NeedInitData => {
                        respond_to_need_init_data(client_message, ws_connection_id).await
                    } // ============================================
                    // ============================================
                    // You have to customize:
                    // - here
                    ClientMessageDiscriminants::Commit => {
                        respond_to_commit(
                            client_message, // , ws_connection_id
                        )
                        .await
                    } // ==========================================
                      // ==========================================
                }
            }
        } {
            send_message_to(message, send, ws_connection_id).await
        }
        post_process(need_send_init_data_after_all, ws_connection_id).await;
    }
}

// ============================================
// ============================================
// You have to customize:
// - here

pub async fn respond_to_commit(
    client_message: ClientMessage,
    // ws_connection_id: WsConnectionId,
) -> Option<(ServerMessage, Send)> {
    if let ClientMessage::Commit(modal) = client_message {
        match ws_send_receive!(RequestMessage::Commit(modal) => ResponseMessage::Commit(res) => res)
        {
            Err(err) => {
                error!("{}:{}: {err:?}", file!(), line!());
                None
            }
            Ok(Err(err)) => Some((ServerMessage::Commit(Err(err.to_string())), Send::ToSelf)),
            Ok(Ok(ret)) => Some((ServerMessage::Commit(Ok(ret)), Send::All)),
        }
    } else {
        unreachable!();
    }
}

// ==========================================
// ==========================================
