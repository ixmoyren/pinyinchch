use std::collections::HashMap;

/// HMM 模型所需的方法
pub trait Hmm {
    /// 获取状态（汉字）的起始概率
    fn start(&self, state: &char) -> f64;

    /// 获取状态（汉字）到观测值（拼音）的发射概率
    fn emission(&self, state: &char, observation: &str) -> f64;

    /// 获取状态转移概率（从一个汉字到另一个汉字）
    fn transition(&self, from_state: &char, to_state: &char) -> f64;

    /// 获取能够产生给定观测值（拼音）的所有状态（汉字）
    fn get_states(&self, observation: &str) -> Vec<char>;
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HmmData {
    pub data: HashMap<char, f64>,
    pub default: f64,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HmmPy2Hz {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub data: HashMap<String, String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HmmTransition {
    pub data: HashMap<char, TransitionEntry>,
    pub default: f64,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct TransitionEntry {
    pub default: f64,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub variants: HashMap<char, f64>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HmmEmission {
    pub data: HashMap<char, HashMap<String, f64>>,
    pub default: f64,
}
