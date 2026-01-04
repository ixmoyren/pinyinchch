use pinyinchch::hmm::viterbi;
use pinyinchch_model_hmm::DefaultHmm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 HMM 实例
    let hmm = DefaultHmm::default();

    // 测试拼音转汉字
    let pinyin_seq = vec!["ni", "hao"];

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
