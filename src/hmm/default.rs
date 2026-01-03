use crate::error::RkyvSnafu;
use crate::hmm::Hmm;
use crate::hmm_dirt_data::{HMM_EMISSION, HMM_PY2HZ, HMM_START, HMM_TRANSITION};
use pinyinchch_type::{HmmData, HmmEmissionData, HmmPy2HzData, HmmTransitionData};
use snafu::ResultExt;
use std::collections::HashMap;

const DEFAULT: &str = "default";

/// 默认 HMM 实现
pub struct DefaultHmm {
    py2hz: HashMap<String, String>,
    start: HmmData,
    emission: HmmEmissionData,
    transition: HmmTransitionData,
}

impl DefaultHmm {
    pub fn try_new() -> crate::Result<Self> {
        let py2hz =
            unsafe { rkyv::from_bytes_unchecked::<HmmPy2HzData, rkyv::rancor::Error>(HMM_PY2HZ) }
                .context(RkyvSnafu {
                file_name: "hmm_py2hz",
            })?;
        let start =
            unsafe { rkyv::from_bytes_unchecked::<HmmData, rkyv::rancor::Error>(HMM_START) }
                .context(RkyvSnafu {
                    file_name: "hmm_start",
                })?;
        let emission = unsafe {
            rkyv::from_bytes_unchecked::<HmmEmissionData, rkyv::rancor::Error>(HMM_EMISSION)
        }
        .context(RkyvSnafu {
            file_name: "hmm_emission",
        })?;
        let transition = unsafe {
            rkyv::from_bytes_unchecked::<HmmTransitionData, rkyv::rancor::Error>(HMM_TRANSITION)
        }
        .context(RkyvSnafu {
            file_name: "hmm_transition",
        })?;
        Ok(Self {
            py2hz: py2hz.data,
            start,
            emission,
            transition,
        })
    }
}

impl Hmm for DefaultHmm {
    fn start(&self, state: &str) -> f64 {
        if let Some(prob) = self.start.data.get(state) {
            *prob
        } else {
            self.start.default
        }
    }

    fn emission(&self, state: &str, observation: &str) -> f64 {
        if let Some(prob_dict) = self.emission.data.get(state)
            && let Some(prob) = prob_dict.get(observation)
        {
            *prob
        } else {
            self.emission.default
        }
    }

    fn transition(&self, from_state: &str, to_state: &str) -> f64 {
        if let Some(prob_dict) = self.transition.data.get(from_state) {
            if let Some(prob) = prob_dict.get(to_state) {
                *prob
            } else if let Some(default_prob) = prob_dict.get(DEFAULT) {
                *default_prob
            } else {
                self.transition.default
            }
        } else {
            self.transition.default
        }
    }

    fn get_states(&self, observation: &str) -> Vec<String> {
        if let Some(hanzi_string) = self.py2hz.get(observation) {
            hanzi_string.chars().map(|c| c.to_string()).collect()
        } else {
            Vec::new()
        }
    }
}
