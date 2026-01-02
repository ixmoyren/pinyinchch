pub mod dag;
mod error;
pub mod hmm;
pub mod pinyin;
mod priority;

pub use error::Error;
type Result<T> = std::result::Result<T, error::Error>;

mod hmm_dirt_data {
    pub(crate) const HMM_EMISSION: &[u8] = include_bytes!("../bin_data/hmm_emission.rkyv");
    pub(crate) const HMM_PY2HZ: &[u8] = include_bytes!("../bin_data/hmm_py2hz.rkyv");
    pub(crate) const HMM_START: &[u8] = include_bytes!("../bin_data/hmm_start.rkyv");
    pub(crate) const HMM_TRANSITION: &[u8] = include_bytes!("../bin_data/hmm_transition.rkyv");
}

mod dag_dirt_data {
    pub(crate) const DAG_CHAR: &[u8] = include_bytes!("../bin_data/dag_char.rkyv");
    pub(crate) const DAG_PHRASE: &[u8] = include_bytes!("../bin_data/dag_phrase.rkyv");
}
