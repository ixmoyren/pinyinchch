use crate::hmm::Hmm;
use crate::hmm_dirt_data::{HMM_EMISSION, HMM_PY2HZ, HMM_START, HMM_TRANSITION};

const DEFAULT: &str = "default";

/// 默认 HMM 实现
#[derive(Default)]
pub struct DefaultHmm {}

impl Hmm for DefaultHmm {
    fn start(&self, state: &str) -> f64 {
        if let Some(prob) = HMM_START.data.get(state) {
            *prob
        } else {
            HMM_START.default
        }
    }

    fn emission(&self, state: &str, observation: &str) -> f64 {
        if let Some(prob_dict) = HMM_EMISSION.data.get(state)
            && let Some(prob) = prob_dict.get(observation)
        {
            *prob
        } else {
            HMM_EMISSION.default
        }
    }

    fn transition(&self, from_state: &str, to_state: &str) -> f64 {
        if let Some(prob_dict) = HMM_TRANSITION.data.get(from_state) {
            if let Some(prob) = prob_dict.get(to_state) {
                *prob
            } else if let Some(default_prob) = prob_dict.get(DEFAULT) {
                *default_prob
            } else {
                HMM_TRANSITION.default
            }
        } else {
            HMM_TRANSITION.default
        }
    }

    fn get_states(&self, observation: &str) -> Vec<String> {
        if let Some(hanzi_string) = HMM_PY2HZ.data.get(observation) {
            hanzi_string.chars().map(|c| c.to_string()).collect()
        } else {
            Vec::new()
        }
    }
}
