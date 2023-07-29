use super::*;

use crate::pasitos::sax3::Sax3RetKey;
use common::send_response_message;
use common_macros2::pasitos;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

#[macro_use]
pub mod common;
pub mod beta;
pub mod ws;

pub fn api() -> impl Filter<
    Extract = (
        impl Reply, /* https://github.com/seanmonstar/warp/issues/646 */
    ),
    Error = Rejection,
> + Clone {
    common::health::api()
        .or(ws::common::api())
        // .or(common::login::api())
        .or(beta::api())
        // ==================================================
        // ==================================================
        // You have to customize:
        // - here
        // ==================================================
        // ==================================================
        .with(warp::trace::request())
}

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display))]
#[strum_discriminants(name(RequestMessageKind))]
pub enum RequestMessage {
    InitData { key: InitDataKey, refresh: bool },
    // ==================================================
    // ==================================================
    // You have to customize:
    // - here
    Commit(Modal),
    Fest(String),
    FestIndex,
    FestJudges,
    // ==================================================
    // ==================================================
}

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display))]
#[strum_discriminants(name(ResponseMessageKind))]
pub enum ResponseMessage {
    InitData(Box<Result<InitData>>),
    // ==================================================
    // ==================================================
    // You have to customize:
    Commit(Result<Modal>),
    Fest(Result<Arc<crate::pasitos::data::FestData>>),
    FestIndex(Result<ResponseMessageFestIndexRet>),
    FestJudges(Result<ResponseMessageFestJudgesRet>),
    // ==================================================
    // ==================================================
}
// use std::collections::HashMap;
pub type ResponseMessageFestIndexRet = Vec<Sax3RetKey>;
pub type ResponseMessageFestJudgesRet = Vec<Arc<crate::pasitos::data::FestData>>;

pub fn process_request_message(request_message: RequestMessage, tx: common::TxHandle) {
    let ret = match request_message {
        RequestMessage::InitData { refresh, key } => {
            if refresh {
                // ==========================================
                // ==========================================
                // You have to customize:
                // - here
                todo!();
                // pasitos!(spreadsheets push_back InitData { tx, key });
                // ==========================================
                // ==========================================
            } else {
                pasitos!(db push_back GetInitData { tx, key });
            }
            None
        } // ==================================================
        // ==================================================
        // You have to customize:
        // - here
        RequestMessage::Commit(modal) => {
            pasitos!(db push_back Commit { tx, modal });
            None
        }
        RequestMessage::FestIndex => {
            pasitos!(data push_back FestIndex { tx });
            None
        }
        RequestMessage::FestJudges => {
            pasitos!(data push_back FestJudges { tx });
            None
        }
        RequestMessage::Fest(fest) => {
            use crate::pasitos::data::{FestDataProxy, FestDataProxyRequest, DATA};
            let data = &mut *DATA.write().unwrap();
            if let Some(ret) = data.get_mut(&fest) {
                match ret {
                    FestDataProxy::Response(data) => {
                        Some((tx, ResponseMessage::Fest(Ok(data.clone()))))
                    }
                    FestDataProxy::Request(FestDataProxyRequest { txlar, .. }) => {
                        txlar.push(tx);
                        None
                    }
                }
            } else {
                data.insert(
                    fest.clone(),
                    FestDataProxy::Request(FestDataProxyRequest {
                        txlar: vec![tx],
                        index_requestlar: vec![],
                        judges_requestlar: vec![],
                    }),
                );
                pasitos!(data push_back Fest { fest });
                None
            }
        } // ================================================
          // ================================================
    };

    if let Some((tx, message)) = ret {
        send_response_message(message, tx);
    }
}
