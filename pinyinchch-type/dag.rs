use std::collections::HashMap;

/// DAG模型所需的方法
pub trait Dag {
    /// 根据拼音列表获取可能的词组和对应的概率, 返回值：Vec<(词组, 概率)>
    fn get_phrase(&self, pinyin_list: &[&str], num: usize) -> Vec<(String, f64)>;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DagChar {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data: HashMap<String, Vec<(String, f64)>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DagPhrase {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data: HashMap<String, Vec<(String, f64)>>,
}
