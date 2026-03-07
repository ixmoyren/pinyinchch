use crate::{HMM_EMISSION, HMM_PY2HZ, HMM_START, HMM_TRANSITION};
use pinyinchch_type::hmm::Hmm;

/// 默认 HMM 实现
#[derive(Default)]
pub struct DefaultHmm {}

impl Hmm for DefaultHmm {
    fn start(&self, state: &char) -> f64 {
        if let Some(prob) = HMM_START.data.get(state) {
            *prob
        } else {
            HMM_START.default
        }
    }

    fn emission(&self, state: &char, observation: &str) -> f64 {
        if let Some(prob_dict) = HMM_EMISSION.data.get(state)
            && let Some(prob) = prob_dict.get(observation)
        {
            *prob
        } else {
            HMM_EMISSION.default
        }
    }

    fn transition(&self, from_state: &char, to_state: &char) -> f64 {
        if let Some(prob_dict) = HMM_TRANSITION.data.get(from_state) {
            if let Some(prob) = prob_dict.variants.get(to_state) {
                *prob
            } else {
                prob_dict.default
            }
        } else {
            HMM_TRANSITION.default
        }
    }

    fn get_states(&self, observation: &str) -> Vec<char> {
        if let Some(hanzi_string) = HMM_PY2HZ.data.get(observation) {
            hanzi_string.chars().collect()
        } else {
            Vec::new()
        }
    }
}
