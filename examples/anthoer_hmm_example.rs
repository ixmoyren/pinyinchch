use pinyinchch::hmm::{DefaultHmm, viterbi};
use pinyinchch::pinyin::pinyin_split_by_trie_tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 HMM 实例
    let hmm = DefaultHmm::default();

    let pinyin = "nihao";

    // 对拼音字符串进行拆分
    let pinyin_split_str = pinyin_split_by_trie_tokenizer(pinyin);

    let pinyin_seq = pinyin_split_str.split(" ").collect::<Vec<_>>();

    // 使用 Viterbi 算法，返回 2 个候选结果，不使用对数概率
    let result = viterbi(&hmm, &pinyin_seq, 2, false, 3.14e-200);

    println!("HMM结果:");
    for item in &result {
        println!("分数: {}, 路径: {:?}", item.score(), item.path());
    }

    // 使用对数概率
    let result_log = viterbi(&hmm, &pinyin_seq, 2, true, 3.14e-200);

    println!("\nHMM结果 (对数概率):");
    for item in &result_log {
        println!("分数: {}, 路径: {:?}", item.score(), item.path());
    }

    Ok(())
}
