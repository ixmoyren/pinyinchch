use pinyinchch::dag::{DefaultDag, dispatch};

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
