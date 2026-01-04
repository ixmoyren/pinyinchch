mod default;

pub use default::*;

use pinyinchch_type::embed_data;
use pinyinchch_type::hmm::{HmmData, HmmEmission, HmmPy2Hz, HmmTransition};

embed_data!(
    HMM_EMISSION,
    HmmEmission,
    HMM_EMISSION_BYTES,
    "bin/hmm_emission.rkyv"
);

embed_data!(HMM_PY2HZ, HmmPy2Hz, HMM_PY2HZ_BYTES, "bin/hmm_py2hz.rkyv");

embed_data!(HMM_START, HmmData, HMM_START_BYTES, "bin/hmm_start.rkyv");

embed_data!(
    HMM_TRANSITION,
    HmmTransition,
    HMM_TRANSITION_BYTES,
    "bin/hmm_transition.rkyv"
);
