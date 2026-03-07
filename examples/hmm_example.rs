use pinyinchch::hmm::viterbi;
use pinyinchch_model_hmm::DefaultHmm;

// HMM结果:
// 分数: 0.000000013155294593897204, 路径: ['你', '知', '不', '知', '道']
// 分数: 0.0000000036677865125992192, 路径: ['你', '只', '不', '知', '道']
//
// HMM结果 (对数概率):
// 分数: -18.14644152864202, 路径: ['你', '知', '不', '知', '道']
// 分数: -19.423677486918002, 路径: ['你', '只', '不', '知', '道']
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 HMM 实例
    let hmm = DefaultHmm::default();

    // 测试拼音转汉字
    let pinyin_seq = vec!["ni", "zhi", "bu", "zhi", "dao"];

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
