mod default;

pub use default::*;

use pinyinchch_type::dag::{DagChar, DagPhrase};
use pinyinchch_type::embed_data;

embed_data!(DAG_CHAR, DagChar, DAG_CHAR_BYTES, "bin/dag_char.rkyv");

embed_data!(
    DAG_PHRASE,
    DagPhrase,
    DAG_PHRASE_BYTES,
    "bin/dag_phrase.rkyv"
);

#[cfg(test)]
mod tests {
    use super::DAG_CHAR_BYTES;
    use pinyinchch_type::dag::DagChar;
    #[test]
    fn test_deserialized_dag_char_from_rkyv_file() {
        let mut aligned = rkyv::util::AlignedVec::<16>::new();
        aligned.extend_from_slice(DAG_CHAR_BYTES);
        let dag_char = rkyv::from_bytes::<DagChar, rkyv::rancor::Error>(&aligned).unwrap();
        let a = dag_char.data.get("a").unwrap();
        assert_eq!(a[1].0, "\u{554a}".to_owned());
    }
}
