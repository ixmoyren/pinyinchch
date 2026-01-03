# pinyinchch

一个拼音转汉字工具库。

## 介绍

这是一个用 Rust 重写的 [Pinyin2Hanzi](https://github.com/letiantian/Pinyin2Hanzi) 库，支持两种模型：

- **HMM (隐马尔可夫模型)**：使用 Viterbi 算法进行拼音转汉字
- **DAG (有向无环图)**：使用动态规划算法进行拼音转汉字

## 安装

```shell
cargo add pinyinchch
```

## 使用方法

### HMM 示例

```rust
use pinyinchch::hmm::{viterbi, DefaultHmm};
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
```

### DAG 示例

```rust
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

```

## 特性

- 支持 HMM 和 DAG 两种转换算法，提供对数概率和线性概率两种评分方式
- 支持自定义 HMM 和 DAG 实现
- 提供将拼音字符串拆分的方法

## 数据文件

本库使用预训练的模型数据，包括：

- `hmm_py2hz.json`: 拼音到汉字的映射
- `hmm_start.json`: 起始概率
- `hmm_emission.json`: 发射概率
- `hmm_transition.json`: 状态转移概率
- `dag_char.json`: 单字拼音数据
- `dag_phrase.json`: 词组拼音数据

## 测试

运行测试：

```bash
cargo test
```

## 许可

许可任你喜欢选择下面任一种，或者两种都选

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### 贡献

除非您另有明确说明，否则任何您提交的代码许可应按上述 Apache 和 MIT 双重许可，并没有任何附加条款或条件。