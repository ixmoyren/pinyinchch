#[cfg(feature = "dag")]
pub mod dag;
mod error;
#[cfg(feature = "hmm")]
pub mod hmm;
pub mod pinyin;
mod priority;

type Result<T> = std::result::Result<T, error::Error>;

#[cfg(feature = "hmm")]
mod hmm_dirt_data {
    use crate::embed_data;
    use pinyinchch_type::{HmmData, HmmEmission, HmmPy2Hz, HmmTransition};

    embed_data!(
        HMM_EMISSION,
        HmmEmission,
        HMM_EMISSION_BYTES,
        "../bin_data/hmm_emission.rkyv"
    );

    embed_data!(
        HMM_PY2HZ,
        HmmPy2Hz,
        HMM_PY2HZ_BYTES,
        "../bin_data/hmm_py2hz.rkyv"
    );

    embed_data!(
        HMM_START,
        HmmData,
        HMM_START_BYTES,
        "../bin_data/hmm_start.rkyv"
    );

    embed_data!(
        HMM_TRANSITION,
        HmmTransition,
        HMM_TRANSITION_BYTES,
        "../bin_data/hmm_transition.rkyv"
    );
}

#[cfg(feature = "dag")]
mod dag_dirt_data {
    use crate::embed_data;
    use pinyinchch_type::{DagChar, DagPhrase};

    embed_data!(
        DAG_CHAR,
        DagChar,
        DAG_CHAR_BYTES,
        "../bin_data/dag_char.rkyv"
    );
    embed_data!(
        DAG_PHRASE,
        DagPhrase,
        DAG_PHRASE_BYTES,
        "../bin_data/dag_phrase.rkyv"
    );

    #[cfg(test)]
    mod tests {
        use super::DAG_CHAR_BYTES;
        use pinyinchch_type::DagChar;
        #[test]
        fn test_deserialized_dag_char_from_rkyv_file() {
            let mut aligned = rkyv::util::AlignedVec::<16>::new();
            aligned.extend_from_slice(DAG_CHAR_BYTES);
            let dag_char = rkyv::from_bytes::<DagChar, rkyv::rancor::Error>(&aligned).unwrap();
            let a = dag_char.data.get("a").unwrap();
            assert_eq!(a[1].0, "\u{554a}".to_owned());
        }
    }
}

#[macro_export]
macro_rules! embed_data {
    ($name:ident,$t:ty,$byte:ident,$path:literal) => {
        pub(crate) const $byte: &'static [u8] = include_bytes!($path);
        pub(crate) static $name: ::std::sync::LazyLock<$t> = ::std::sync::LazyLock::new(|| {
            let mut aligned = rkyv::util::AlignedVec::<16>::new();
            aligned.extend_from_slice($byte);
            rkyv::from_bytes::<$t, rkyv::rancor::Error>(&aligned).expect(concat!(
                "Failed to crate ",
                stringify!($name),
                "stringify!($name)",
                $path
            ))
        });
    };
}
