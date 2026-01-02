use crate::dag::Dag;
use crate::dag_dirt_data::{DAG_CHAR, DAG_PHRASE};
use crate::error::RkyvSnafu;
use pinyinchch_type::{DagCharData, DagPhraseData};
use snafu::ResultExt;
use std::collections::HashMap;

/// 默认DAG参数实现
pub struct DefaultDag {
    char: HashMap<String, Vec<(String, f64)>>,
    phrase: HashMap<String, Vec<(String, f64)>>,
}

impl DefaultDag {
    pub fn try_new() -> crate::Result<Self> {
        let char =
            unsafe { rkyv::from_bytes_unchecked::<DagCharData, rkyv::rancor::Error>(DAG_CHAR) }
                .context(RkyvSnafu {
                    file_name: "dag_char",
                })?;
        let phrase =
            unsafe { rkyv::from_bytes_unchecked::<DagPhraseData, rkyv::rancor::Error>(DAG_PHRASE) }
                .context(RkyvSnafu {
                    file_name: "dag_phrase",
                })?;

        Ok(DefaultDag {
            char: char.data,
            phrase: phrase.data,
        })
    }
}

impl Dag for DefaultDag {
    fn get_phrase(&self, pinyin_list: &[&str], num: usize) -> Vec<(String, f64)> {
        if pinyin_list.is_empty() {
            return Vec::new();
        }

        let pinyin_key = pinyin_list.join(",");

        let data = if pinyin_list.len() == 1 {
            self.char.get(&pinyin_key)
        } else {
            self.phrase.get(&pinyin_key)
        };

        if let Some(phrase_data) = data {
            let take_num = std::cmp::min(num, phrase_data.len());
            phrase_data[..take_num].to_vec()
        } else {
            Vec::new()
        }
    }
}
