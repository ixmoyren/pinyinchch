use crate::dag::Dag;
use crate::priority::{Item, PrioritySet};

/// DAG算法实现拼音转汉字
///
/// # Arguments
/// * `dag` - DAG参数实现
/// * `pinyin_seq` - 拼音列表
/// * `path_num` - 返回路径数量
/// * `use_log_prob` - 是否使用对数概率
pub fn dispatch(
    dag: &impl Dag,
    pinyin_seq: &[&str],
    path_num: usize,
    use_log_prob: bool,
) -> Vec<Item> {
    if pinyin_seq.is_empty() {
        return Vec::new();
    }
    let pinyin_num = pinyin_seq.len();

    // 创建动态规划数组
    let mut dispatch_vec = Vec::with_capacity(pinyin_num);
    for _ in 0..pinyin_num {
        dispatch_vec.push(PrioritySet::new(path_num));
    }

    // 处理起始位置（from_idx = 0）
    for from_idx in 0..1 {
        for to_idx in from_idx..pinyin_num {
            let slice = &pinyin_seq[from_idx..to_idx + 1];

            let phrase_prob_pairs = dag.get_phrase(slice, path_num);
            for (phrase, prob) in phrase_prob_pairs {
                let word = vec![phrase];
                let score = if use_log_prob { prob.ln() } else { prob };
                dispatch_vec[to_idx].put(score, word);
            }
        }
    }

    // 处理后续位置（from_idx >= 1）
    for from_idx in 1..pinyin_num {
        // 先收集前一个位置的数据，避免借用冲突
        let prev_items: Vec<_> = dispatch_vec[from_idx - 1]
            .iter()
            .map(|item| (item.score(), item.path().clone()))
            .collect();

        for to_idx in from_idx..pinyin_num {
            let slice = &pinyin_seq[from_idx..to_idx + 1];

            let phrase_prob_pairs = dag.get_phrase(slice, path_num);
            for (prev_score, prev_path) in &prev_items {
                for (phrase, prob) in &phrase_prob_pairs {
                    let mut word = prev_path.clone();
                    word.push(phrase.clone());

                    let score = if use_log_prob {
                        *prev_score + prob.ln()
                    } else {
                        *prev_score * prob
                    };

                    dispatch_vec[to_idx].put(score, word);
                }
            }
        }
    }

    // 获取最终结果
    let mut result: Vec<Item> = dispatch_vec.last().unwrap().to_sorted_vec();
    result.sort_by(|a, b| {
        b.score()
            .partial_cmp(&a.score())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // 为测试创建一个简单的DAG参数实现
    struct TestDag {
        data: HashMap<String, Vec<(String, f64)>>,
    }

    impl TestDag {
        fn new() -> Self {
            let mut data = HashMap::new();
            data.insert(
                "ni".to_string(),
                vec![("你".to_string(), 0.8), ("泥".to_string(), 0.2)],
            );
            data.insert(
                "hao".to_string(),
                vec![("好".to_string(), 0.7), ("号".to_string(), 0.3)],
            );
            data.insert("ni,hao".to_string(), vec![("你好".to_string(), 0.9)]);

            TestDag { data }
        }
    }

    impl Dag for TestDag {
        fn get_phrase(&self, pinyin_list: &[&str], num: usize) -> Vec<(String, f64)> {
            let key = pinyin_list.join(",");
            if let Some(values) = self.data.get(&key) {
                let take_num = std::cmp::min(num, values.len());
                values[..take_num].to_vec()
            } else {
                Vec::new()
            }
        }
    }

    #[test]
    fn test_dag() {
        let params = TestDag::new();
        let pinyin_list = vec!["ni", "hao"];
        let result = dispatch(&params, &pinyin_list, 2, false);
        assert!(!result.is_empty());
    }
}
