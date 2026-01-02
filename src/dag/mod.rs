mod default;
mod dispatch;

pub use default::DefaultDag;
pub use dispatch::dispatch;

/// DAG模型所需的方法
pub trait Dag {
    /// 根据拼音列表获取可能的词组和对应的概率, 返回值：Vec<(词组, 概率)>
    fn get_phrase(&self, pinyin_list: &[&str], num: usize) -> Vec<(String, f64)>;
}
