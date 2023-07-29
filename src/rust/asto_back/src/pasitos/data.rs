use super::*;

use crate::pasitos::sax3::*;
use crate::pasitos::spreadsheet::utils::CompetKind;
use crate::server::{
    common::{send_response_message, TxHandle},
    ResponseMessage,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

pub type Compets = Vec<(CompetKind, Sax3Compet)>;

pub enum FestDataProxy {
    Response(std::sync::Arc<FestData>),
    Request(FestDataProxyRequest),
}

#[derive(Default)]
pub struct FestDataProxyRequest {
    pub txlar: Vec<TxHandle>,
    pub index_requestlar: Vec<FestIndexRequestWrapped>,
    pub judges_requestlar: Vec<FestJudgesRequestWrapped>,
}

#[derive(Clone, Debug)]
pub struct FestData {
    pub ret_key: Sax3RetKey,
    pub beginner_compets: Compets,
    pub non_beginner_compets: Compets,
    pub dancerlar: FestDancerlar,
    pub judgelar: FestJudgelar,
}

use std::collections::BTreeMap;

pub type FestDancerlar = BTreeMap<(String, String), HashMap<FestDancer, Vec<(String, usize)>>>;
type GetDancerlarRet = HashMap<(String, String), HashMap<FestDancer, Vec<(String, usize)>>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FestDancer {
    pub first_name: String,
    pub last_name: String,
    pub book_number: Option<i32>,
    pub birth_day: Option<chrono::NaiveDate>,
    pub class: Option<String>,
    pub club: Sax3Club,
}

pub type FestJudgelarKey = (String, String); // (last_name, first_name)
pub type FestJudgelarValueItem = (String, Option<String>, usize); //(category, round_name_opt, couples_len
                                                                  //
pub type FestJudgelar = BTreeMap<FestJudgelarKey, HashMap<Sax3Judge, Vec<FestJudgelarValueItem>>>;

type GetJudgelarRet = HashMap<FestJudgelarKey, HashMap<Sax3Judge, Vec<FestJudgelarValueItem>>>;

lazy_static::lazy_static! {
    pub static ref DATA: std::sync::RwLock<HashMap<String, FestDataProxy>> = std::sync::RwLock::new(HashMap::new());
}

// ==========================================================================

pub type FestJudgesResult = Result<Vec<String>>;
pub async fn fest_judges() -> FestJudgesResult {
    let base_path = Path::new("xml");
    let mut dir = tokio::fs::read_dir(&base_path)
        .await
        .map_err(|err| anyhow!("read_dir{:?}: {}", base_path, err))?;
    let mut ret = vec![];
    while let Some(entry) = dir.next_entry().await? {
        if entry.metadata().await?.is_dir() {
            let fest = entry.file_name().to_string_lossy().to_string();
            let file_pathlar = get_file_pathlar(&fest);
            if file_pathlar.is_empty() {
                continue;
            }
            ret.push(fest);
        }
    }
    Ok(ret)
}
pub fn fest_judges_sync(res: FestJudgesResult, tx: TxHandle) -> Result<()> {
    match res {
        Err(err) => {
            send_response_message(ResponseMessage::FestJudges(Err(err)), tx);
        }
        Ok(festlar) => {
            let data = &mut *DATA.write().unwrap();
            let judges_request = FestJudgesRequestWrapped::new(tx);
            for fest in festlar {
                if let Some(ret) = data.get_mut(&fest) {
                    match ret {
                        FestDataProxy::Response(data) => {
                            judges_request.did_process(data);
                        }
                        FestDataProxy::Request(FestDataProxyRequest {
                            judges_requestlar, ..
                        }) => {
                            judges_requestlar.push(judges_request.clone());
                        }
                    }
                } else {
                    data.insert(
                        fest.clone(),
                        FestDataProxy::Request(FestDataProxyRequest {
                            txlar: vec![],
                            judges_requestlar: vec![judges_request.clone()],
                            ..FestDataProxyRequest::default()
                        }),
                    );
                    pasitos!(data push_back Fest { fest });
                }
            }
            if judges_request.is_ready() {
                judges_request.finish();
            }
        }
    }
    Ok(())
}

// ==========================================================================

pub type FestIndexResult = Result<Vec<String>>;
pub async fn fest_index() -> FestIndexResult {
    let base_path = Path::new("xml");
    let mut dir = tokio::fs::read_dir(&base_path)
        .await
        .map_err(|err| anyhow!("read_dir{:?}: {}", base_path, err))?;
    let mut ret = vec![];
    while let Some(entry) = dir.next_entry().await? {
        if entry.metadata().await?.is_dir() {
            let fest = entry.file_name().to_string_lossy().to_string();
            let file_pathlar = get_file_pathlar(&fest);
            if file_pathlar.is_empty() {
                continue;
            }
            ret.push(fest);
        }
    }
    Ok(ret)
}
pub fn fest_index_sync(res: FestIndexResult, tx: TxHandle) -> Result<()> {
    match res {
        Err(err) => {
            send_response_message(ResponseMessage::FestIndex(Err(err)), tx);
        }
        Ok(festlar) => {
            let data = &mut *DATA.write().unwrap();
            let index_request = FestIndexRequestWrapped::new(tx);
            for fest in festlar {
                if let Some(ret) = data.get_mut(&fest) {
                    match ret {
                        FestDataProxy::Response(data) => {
                            index_request.did_process(data);
                        }
                        FestDataProxy::Request(FestDataProxyRequest {
                            index_requestlar, ..
                        }) => {
                            index_requestlar.push(index_request.clone());
                        }
                    }
                } else {
                    data.insert(
                        fest.clone(),
                        FestDataProxy::Request(FestDataProxyRequest {
                            txlar: vec![],
                            index_requestlar: vec![index_request.clone()],
                            ..FestDataProxyRequest::default()
                        }),
                    );
                    pasitos!(data push_back Fest { fest });
                }
            }
            if index_request.is_ready() {
                index_request.finish();
            }
        }
    }
    Ok(())
}

// ==========================================================================

#[derive(Clone)]
pub struct FestJudgesRequestWrapped(Arc<RwLock<FestJudgesRequest>>);
pub struct FestJudgesRequest {
    ret: crate::server::ResponseMessageFestJudgesRet,
    tx: Option<TxHandle>,
}

impl FestJudgesRequestWrapped {
    pub fn new(tx: TxHandle) -> Self {
        Self(Arc::new(RwLock::new(FestJudgesRequest {
            ret: crate::server::ResponseMessageFestJudgesRet::new(),
            tx: Some(tx),
        })))
    }
    pub fn is_ready(&self) -> bool {
        Arc::strong_count(&self.0) <= 1
    }
    pub fn did_process(&self, data: &Arc<FestData>) {
        self.0.write().unwrap().ret.push(data.clone());
    }
    pub fn finish_with_err(&self, err: &Error) {
        if let Some(tx) = self.0.write().unwrap().tx.take() {
            send_response_message(ResponseMessage::FestJudges(Err(anyhow!("{err}"))), tx);
        }
    }
    pub fn finish(self) {
        let v = &mut self.0.write().unwrap();
        if let Some(tx) = v.tx.take() {
            send_response_message(ResponseMessage::FestJudges(Ok(v.ret.clone())), tx);
        }
    }
}

// ==========================================================================

#[derive(Clone)]
pub struct FestIndexRequestWrapped(Arc<RwLock<FestIndexRequest>>);
pub struct FestIndexRequest {
    ret: crate::server::ResponseMessageFestIndexRet,
    tx: Option<TxHandle>,
}

impl FestIndexRequestWrapped {
    pub fn new(tx: TxHandle) -> Self {
        Self(Arc::new(RwLock::new(FestIndexRequest {
            ret: crate::server::ResponseMessageFestIndexRet::new(),
            tx: Some(tx),
        })))
    }
    pub fn is_ready(&self) -> bool {
        Arc::strong_count(&self.0) <= 1
    }
    pub fn did_process(&self, data: &Arc<FestData>) {
        self.0.write().unwrap().ret.push(data.ret_key.clone());
    }
    pub fn finish_with_err(&self, err: &Error) {
        if let Some(tx) = self.0.write().unwrap().tx.take() {
            send_response_message(ResponseMessage::FestIndex(Err(anyhow!("{err}"))), tx);
        }
    }
    pub fn finish(self) {
        let v = &mut self.0.write().unwrap();
        if let Some(tx) = v.tx.take() {
            send_response_message(ResponseMessage::FestIndex(Ok(v.ret.clone())), tx);
        }
    }
}

// ==========================================================================

pub fn get_file_pathlar(fest: &str) -> Vec<std::path::PathBuf> {
    let base_path = Path::new("xml").join(Path::new(&fest));
    let file_path = base_path.join(Path::new("s6_edited.xml"));
    if file_path.exists() {
        vec![file_path]
    } else {
        let file_path = base_path.join(Path::new("s6.xml"));
        if file_path.exists() {
            vec![file_path]
        } else {
            let mut ret = vec![];
            let file_path = base_path.join(Path::new("smm_edited.xml"));
            if file_path.exists() {
                ret.push(file_path);
            } else {
                let file_path = base_path.join(Path::new("smm.xml"));
                if file_path.exists() {
                    ret.push(file_path);
                }
            }
            let file_path = base_path.join(Path::new("svd_edited.xml"));
            if file_path.exists() {
                ret.push(file_path);
            } else {
                let file_path = base_path.join(Path::new("svd.xml"));
                if file_path.exists() {
                    ret.push(file_path);
                }
            }
            ret
        }
    }
}

pub type FestResult = Result<FestData>;
pub async fn fest(fest: &str) -> FestResult {
    let file_pathlar = get_file_pathlar(fest);

    let ret = crate::pasitos::sax3::sax3(&file_pathlar)?;
    if ret.len() != 1 {
        bail!("ret.keys(): {:?}", ret.keys());
    }
    let (ret_key, compets) = ret.into_iter().next().unwrap();

    type Gradient = String;
    type Dance = String;
    type Dances = Vec<Dance>;
    type Phase = String;
    type PhasesByDance = HashMap<Dance, Vec<(Phase, Sax3Compet)>>;

    let gradient_category_prefixlar: HashMap<&str, Vec<&str>> = vec![
        (
            "2022-11-06",
            vec![
                "Соло, Юниоры-1, Юниоры-2, Молодежь",
                "Молодежь, Взрослые, Сеньоры",
            ],
        ),
        ("2023-03-19", vec!["Градиент"]),
    ]
    .into_iter()
    .collect();
    let skip_category_prefixlar: HashMap<&str, Vec<&str>> = vec![(
        "2023-03-19",
        vec![
            "Шербургские зонтики",
            "Ветер перемен",
            "Зажигалки",
            "Move",
            "Загадка",
            "Соло, Сеньоры",
        ],
    )]
    .into_iter()
    .collect();

    let mut gradient_compets: HashMap<Gradient, (Dances, PhasesByDance)> = HashMap::new();
    let mut non_gradient_compets = vec![];

    'COMPETS: for compet in compets
        .into_iter()
        .filter(|compet| !compet.couples.is_empty())
    {
        if let Some(skip_category_prefixlar) =
            skip_category_prefixlar.get(ret_key.date.to_string().as_str())
        {
            for skip_category_prefix in skip_category_prefixlar.iter() {
                if let Some(_category) = compet.category.strip_prefix(skip_category_prefix) {
                    continue 'COMPETS;
                }
            }
        }

        let non_gradient_compet = {
            let mut ret = Some(compet);
            if let Some(gradient_category_prefixlar) =
                gradient_category_prefixlar.get(ret_key.date.to_string().as_str())
            {
                for gradient in gradient_category_prefixlar.iter() {
                    if let Some(category) = ret.as_ref().unwrap().category.strip_prefix(gradient) {
                        let category = category.trim_matches(|ch| matches!(ch, ' ' | ','));
                        let splitted = category.split('(').collect::<Vec<_>>();
                        let dance = splitted[0].trim().to_owned();
                        let phase = splitted[1].trim_end_matches(')').to_owned();
                        let gradient = gradient.to_string();
                        let compet = ret.take().unwrap();
                        common_macros2::entry!(gradient_compets, gradient
                        =>
                            and_modify |e| {
                                e.0.push(dance.clone());
                                common_macros2::entry!(e.1, dance
                                =>
                                    and_modify |e| {
                                        e.push((phase, compet));
                                    }
                                    or_insert vec![(phase, compet)]
                                );
                            }
                            or_insert (vec![dance.clone()], vec![(dance, vec![(phase, compet)])].into_iter().collect())
                        );
                        break;
                    }
                }
            }
            ret
        };
        if let Some(compet) = non_gradient_compet {
            non_gradient_compets.push(compet);
        }
    }

    let mut beginner_compets = vec![];
    let mut non_beginner_compets = vec![];
    {
        for compet in non_gradient_compets {
            let kind = crate::pasitos::spreadsheet::get_kind_of_category(&compet.category);
            match kind {
                CompetKind::Кубок | CompetKind::Аттестация => {
                    beginner_compets.push((kind, compet));
                }
                CompetKind::Категория => {
                    non_beginner_compets.push((kind, compet));
                }
            }
        }
    }
    let data = FestData {
        dancerlar: get_dancerlar(&[&beginner_compets, &non_beginner_compets])
            .into_iter()
            .collect::<FestDancerlar>(),
        judgelar: get_judgelar(&[&beginner_compets, &non_beginner_compets])
            .into_iter()
            .collect::<FestJudgelar>(),
        ret_key,
        beginner_compets,
        non_beginner_compets,
    };
    Ok(data)
}

pub fn fest_sync(res: FestResult, fest: String) -> Result<()> {
    let data = &mut *DATA.write().unwrap();
    let res = res.map(std::sync::Arc::new);
    let removed = data.remove(&fest);
    if let Ok(ref ret) = res {
        data.insert(fest, FestDataProxy::Response(ret.clone()));
    }
    if let Some(FestDataProxy::Request(FestDataProxyRequest {
        txlar,
        index_requestlar,
        judges_requestlar,
    })) = removed
    {
        for tx in txlar {
            send_response_message(
                ResponseMessage::Fest(match &res {
                    Err(err) => Err(anyhow!("{err}")),
                    Ok(data) => Ok(data.clone()),
                }),
                tx,
            );
        }
        for index_request in index_requestlar {
            match &res {
                Err(err) => index_request.finish_with_err(err),
                Ok(data) => {
                    index_request.did_process(data);
                    if index_request.is_ready() {
                        index_request.finish();
                    }
                }
            }
        }
        for judges_request in judges_requestlar {
            match &res {
                Err(err) => judges_request.finish_with_err(err),
                Ok(data) => {
                    judges_request.did_process(data);
                    if judges_request.is_ready() {
                        judges_request.finish();
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_judgelar(competslar: &[&Compets]) -> GetJudgelarRet {
    let mut ret: GetJudgelarRet = GetJudgelarRet::new();
    for compets in competslar {
        for (
            _,
            Sax3Compet {
                category,
                judges,
                rounds,
                couples,
            },
        ) in (**compets).iter()
        {
            let couples_len = couples.len();
            if let Some(Sax3Judges(judges)) = judges {
                for (_, judge) in judges.iter() {
                    upsert_get_judgelar_ret(&mut ret, category, None, couples_len, judge);
                }
            } else {
                for (_, Sax3Round { name, judges, .. }) in rounds.iter() {
                    if let Some(Sax3Judges(judges)) = &judges {
                        for (_, judge) in judges.iter() {
                            upsert_get_judgelar_ret(
                                &mut ret,
                                category,
                                Some(name),
                                couples_len,
                                judge,
                            );
                        }
                        // let value = (category.clone(), Some(name.clone()), couples.len());
                        // for (_, judge) in judges.iter() {
                        //     entry!(ret, judge.clone()
                        //     =>
                        //         and_modify |e| { e.push(value.clone()) }
                        //         or_insert vec![value.clone()]
                        //     )
                        // }
                    }
                }
            }
        }
    }
    ret
}

fn get_dancerlar(competslar: &[&Compets]) -> GetDancerlarRet {
    let mut ret: GetDancerlarRet = GetDancerlarRet::new();
    for compets in competslar.iter() {
        for (
            _compet_kind,
            Sax3Compet {
                category, couples, ..
            },
        ) in compets.iter()
        {
            for (
                _couple_number,
                Sax3Couple {
                    male,
                    female,
                    club,
                    class: couple_class,
                    ..
                },
            ) in couples.iter()
            {
                let couples_len = couples.len();
                upsert_get_dancerlar_ret(&mut ret, category, couples_len, club, couple_class, male);
                if let Some(female) = female {
                    upsert_get_dancerlar_ret(
                        &mut ret,
                        category,
                        couples_len,
                        club,
                        couple_class,
                        female,
                    );
                }
            }
        }
    }
    ret
}

fn upsert_get_dancerlar_ret(
    ret: &mut GetDancerlarRet,
    category: &str,
    couples_len: usize,
    club: &Sax3Club,
    couple_class: &Option<String>,
    Sax3Dancer {
        first_name,
        last_name,
        book_number,
        birth_day,
        class,
        ..
    }: &Sax3Dancer,
) {
    let dancer = FestDancer {
        first_name: first_name.to_owned(),
        last_name: last_name.to_owned(),
        book_number: *book_number,
        birth_day: *birth_day,
        class: class.clone().or_else(|| couple_class.clone()),
        club: club.clone(),
    };
    let category = category.to_owned();
    let value = (category, couples_len);
    entry!(ret, ( last_name.clone() , first_name.clone() )
    =>
        and_modify |e| {
            entry!(e, dancer
            =>
                and_modify |e| { e.push(value) }
                or_insert vec![value]
            );
        }
        or_insert HashMap::from([(dancer, vec![value])])
    );
}

fn upsert_get_judgelar_ret(
    ret: &mut GetJudgelarRet,
    category: &str,
    round_name_opt: Option<&String>,
    couples_len: usize,
    judge: &Sax3Judge,
) {
    let category = category.to_owned();
    let value = (category, round_name_opt.cloned(), couples_len);
    let judge = judge.clone();
    entry!(ret, ( judge.last_name.clone(), judge.first_name.clone())
    =>
        and_modify |e| {
            entry!(e, judge
            =>
                and_modify |e| { e.push(value) }
                or_insert vec![value]
            );
        }
        or_insert HashMap::from([(judge, vec![value])])
    );
}
