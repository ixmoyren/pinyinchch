mod default;
mod viterbi;

pub use default::DefaultHmm;
pub use viterbi::viterbi;

/// HMM 模型所需的方法
pub trait Hmm {
    /// 获取状态（汉字）的起始概率
    fn start(&self, state: &str) -> f64;

    /// 获取状态（汉字）到观测值（拼音）的发射概率
    fn emission(&self, state: &str, observation: &str) -> f64;

    /// 获取状态转移概率（从一个汉字到另一个汉字）
    fn transition(&self, from_state: &str, to_state: &str) -> f64;

    /// 获取能够产生给定观测值（拼音）的所有状态（汉字）
    fn get_states(&self, observation: &str) -> Vec<String>;
}
