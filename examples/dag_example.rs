use pinyinchch::dag::dispatch;
use pinyinchch_model_dag::DefaultDag;

// DAG结果:
// 分数: 0.21953546530016635, 路径: ["你好"]
// 分数: 0.015182124223139684, 路径: ["你", "好"]
//
// DAG结果 (对数概率):
// 分数: -1.516241486396955, 路径: ["你好"]
// 分数: -4.187636581156532, 路径: ["你", "好"]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建DAG参数实例
    let dag_params = DefaultDag::default();

    // 测试拼音转汉字
    let pinyin_list = vec!["ni", "hao"];

    // 使用DAG算法，返回2个候选结果，不使用对数概率
    let result = dispatch(&dag_params, &pinyin_list, 2, false);

    println!("DAG结果:");
    for item in &result {
        println!("分数: {}, 路径: {:?}", item.score(), item.path());
    }

    // 使用对数概率
    let result_log = dispatch(&dag_params, &pinyin_list, 2, true);

    println!("\nDAG结果 (对数概率):");
    for item in &result_log {
        println!("分数: {}, 路径: {:?}", item.score(), item.path());
    }

    Ok(())
}
