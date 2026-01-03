use crate::dag::Dag;
use crate::dag_dirt_data::{DAG_CHAR, DAG_PHRASE};

/// 默认DAG参数实现
/// #[derive(Default)]
pub struct DefaultDag {}

impl Dag for DefaultDag {
    fn get_phrase(&self, pinyin_list: &[&str], num: usize) -> Vec<(String, f64)> {
        if pinyin_list.is_empty() {
            return Vec::new();
        }

        let pinyin_key = pinyin_list.join(",");

        let data = if pinyin_list.len() == 1 {
            DAG_CHAR.data.get(&pinyin_key)
        } else {
            DAG_PHRASE.data.get(&pinyin_key)
        };

        if let Some(phrase_data) = data {
            let take_num = std::cmp::min(num, phrase_data.len());
            phrase_data[..take_num].to_vec()
        } else {
            Vec::new()
        }
    }
}
