use crate::priority::{Item, PrioritySet};
use pinyinchch_type::hmm::Hmm;
use std::collections::HashMap;

/// 使用 Viterbi 算法，实现拼音转汉字
///
/// * `hmm` - HMM 实现
/// * `pinyin_seq` - 需要转换的拼音序列
/// * `path_num` - 返回路径数量
/// * `use_log_prob` - 是否使用对数概率
/// * `min_prob` - 最小概率值，防止概率为0
pub fn viterbi(
    hmm: &impl Hmm,
    pinyin_seq: &[&str],
    path_num: usize,
    use_log_prob: bool,
    min_prob: f64,
) -> Vec<Item<char>> {
    if pinyin_seq.is_empty() {
        return Vec::new();
    }

    // 存储到达时刻 time 状态 state 的最优路径
    let mut time_and_state = Vec::<HashMap<char, PrioritySet<char>>>::new();

    // 初始化基础情况 (t == 0)
    let cur_obs = pinyin_seq[0];
    let cur_states = hmm.get_states(cur_obs);
    let mut initial_map = HashMap::new();
    for state in &cur_states {
        let start_prob = hmm.start(state);
        let emission_prob = hmm.emission(state, cur_obs);
        let score = if use_log_prob {
            f64::max(start_prob, min_prob).ln() + f64::max(emission_prob, min_prob).ln()
        } else {
            f64::max(start_prob, min_prob) * f64::max(emission_prob, min_prob)
        };

        let path = vec![*state];
        let mut ps = PrioritySet::new(path_num);
        ps.put(score, path);
        initial_map.insert(*state, ps);
    }
    time_and_state.push(initial_map);

    // 运行 t > 0 的 Viterbi 算法
    let mut prev_states = cur_states;
    for &cur_obs in pinyin_seq.iter().skip(1) {
        // 只保留前一个时刻的结果
        if time_and_state.len() == 2
            && let Some(last_time_and_state) = time_and_state.pop()
        {
            time_and_state = vec![last_time_and_state];
        }
        let mut next_map = HashMap::new();
        let cur_states = hmm.get_states(cur_obs);

        for y in &cur_states {
            let mut ps = PrioritySet::new(path_num);

            for y0 in &prev_states {
                if let Some(prev_ps) = time_and_state[0].get(y0) {
                    for item in prev_ps.iter() {
                        let transition_prob = hmm.transition(y0, y);
                        let emission_prob = hmm.emission(y, cur_obs);

                        let new_score = if use_log_prob {
                            item.score()
                                + f64::max(transition_prob, min_prob).ln()
                                + f64::max(emission_prob, min_prob).ln()
                        } else {
                            item.score()
                                * f64::max(transition_prob, min_prob)
                                * f64::max(emission_prob, min_prob)
                        };

                        let mut new_path = item.path().clone();
                        new_path.push(*y);

                        ps.put(new_score, new_path);
                    }
                }
            }

            next_map.insert(*y, ps);
        }

        time_and_state.push(next_map);
        prev_states = cur_states;
    }

    // 收集最终结果
    let mut result = PrioritySet::new(path_num);
    if let Some(final_map) = time_and_state.last() {
        for ps in final_map.values() {
            for item in ps.iter() {
                result.put(item.score(), item.path().clone());
            }
        }
    }

    let mut result_vec: Vec<Item<char>> = result.to_sorted_vec();
    result_vec.sort_by(|a, b| {
        b.score()
            .partial_cmp(&a.score())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    result_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    // 为测试创建一个简单的HMM参数实现
    #[allow(unused)]
    struct TestHmm {
        states: Vec<String>,
        start_probs: HashMap<char, f64>,
        emission_probs: HashMap<char, HashMap<String, f64>>,
        transition_probs: HashMap<char, HashMap<char, f64>>,
        py2hz_map: HashMap<String, String>,
    }

    impl TestHmm {
        fn new() -> Self {
            let mut start_probs = HashMap::new();
            start_probs.insert('你', 0.6);
            start_probs.insert('我', 0.4);

            let mut emission_probs = HashMap::new();
            let mut ni_emissions = HashMap::new();
            ni_emissions.insert("ni".to_string(), 0.8);
            ni_emissions.insert("hello".to_string(), 0.2);
            emission_probs.insert('你', ni_emissions);

            let mut wo_emissions = HashMap::new();
            wo_emissions.insert("wo".to_string(), 0.7);
            wo_emissions.insert("hello".to_string(), 0.3);
            emission_probs.insert('我', wo_emissions);

            let mut transition_probs = HashMap::new();
            let mut ni_transitions = HashMap::new();
            ni_transitions.insert('我', 0.3);
            ni_transitions.insert('你', 0.7);
            transition_probs.insert('你', ni_transitions);

            let mut wo_transitions = HashMap::new();
            wo_transitions.insert('我', 0.6);
            wo_transitions.insert('你', 0.4);
            transition_probs.insert('我', wo_transitions);

            let mut py2hz_map = HashMap::new();
            py2hz_map.insert("ni".to_string(), "你".to_string());
            py2hz_map.insert("wo".to_string(), "我".to_string());

            TestHmm {
                states: vec!["你".to_string(), "我".to_string()],
                start_probs,
                emission_probs,
                transition_probs,
                py2hz_map,
            }
        }
    }

    impl Hmm for TestHmm {
        fn start(&self, state: &char) -> f64 {
            self.start_probs.get(state).copied().unwrap_or(0.0)
        }

        fn emission(&self, state: &char, observation: &str) -> f64 {
            if let Some(state_map) = self.emission_probs.get(state) {
                state_map.get(observation).copied().unwrap_or(0.0)
            } else {
                0.0
            }
        }

        fn transition(&self, from_state: &char, to_state: &char) -> f64 {
            if let Some(from_map) = self.transition_probs.get(from_state) {
                from_map.get(to_state).copied().unwrap_or(0.0)
            } else {
                0.0
            }
        }

        fn get_states(&self, observation: &str) -> Vec<char> {
            if let Some(hanzi_string) = self.py2hz_map.get(observation) {
                hanzi_string.chars().collect()
            } else {
                Vec::new()
            }
        }
    }

    #[test]
    fn test_viterbi() {
        let params = TestHmm::new();
        let observations = vec!["ni"];

        let result = viterbi(&params, &observations, 2, false, 3.14e-200);

        assert!(!result.is_empty());
        assert_eq!(result[0].path(), &vec!['你']);
    }
}
