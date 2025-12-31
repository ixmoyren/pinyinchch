use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// 带声调的韵母和和不带声调的韵母的映射
static TONE_TO_PLAIN: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
    HashMap::from([
        ('ā', 'a'),
        ('á', 'a'),
        ('ǎ', 'a'),
        ('à', 'a'),
        ('ē', 'e'),
        ('é', 'e'),
        ('ě', 'e'),
        ('è', 'e'),
        ('ế', 'e'),
        ('ề', 'e'),
        ('ê', 'e'),
        ('ō', 'o'),
        ('ó', 'o'),
        ('ǒ', 'o'),
        ('ò', 'o'),
        ('ī', 'i'),
        ('í', 'i'),
        ('ǐ', 'i'),
        ('ì', 'i'),
        ('ū', 'u'),
        ('ú', 'u'),
        ('ǔ', 'u'),
        ('ù', 'u'),
        ('ǘ', 'u'),
        ('ǚ', 'u'),
        ('ǜ', 'u'),
        ('ü', 'u'),
        ('ń', 'n'),
        ('ň', 'n'),
        ('ǹ', 'n'),
        ('ḿ', 'm'),
    ])
});

static VALID_PINYIN: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "a", "ai", "an", "ang", "ao", "ba", "bai", "ban", "bang", "bao", "bei", "ben", "beng",
        "bi", "bian", "biao", "bie", "bin", "bing", "bo", "bu", "ca", "cai", "can", "cang", "cao",
        "ce", "cen", "ceng", "cha", "chai", "chan", "chang", "chao", "che", "chen", "cheng", "chi",
        "chong", "chou", "chu", "chuai", "chuan", "chuang", "chui", "chun", "chuo", "ci", "cong",
        "cou", "cu", "cuan", "cui", "cun", "cuo", "da", "dai", "dan", "dang", "dao", "de", "dei",
        "den", "deng", "di", "dia", "dian", "diao", "die", "ding", "diu", "dong", "dou", "du",
        "duan", "dui", "dun", "duo", "e", "ei", "en", "eng", "er", "fa", "fan", "fang", "fei",
        "fen", "feng", "fo", "fou", "fu", "ga", "gai", "gan", "gang", "gao", "ge", "gei", "gen",
        "geng", "gong", "gou", "gu", "gua", "guai", "guan", "guang", "gui", "gun", "guo", "ha",
        "hai", "han", "hang", "hao", "he", "hei", "hen", "heng", "hong", "hou", "hu", "hua",
        "huai", "huan", "huang", "hui", "hun", "huo", "ji", "jia", "jian", "jiang", "qiao", "jiao",
        "jie", "jin", "jing", "jiong", "jiu", "ju", "juan", "jue", "jun", "jv", "ka", "kai", "kan",
        "kang", "kao", "ke", "kei", "ken", "keng", "kong", "kou", "ku", "kua", "kuai", "kuan",
        "kuang", "kui", "kun", "kuo", "la", "lai", "lan", "lang", "lao", "le", "lei", "leng", "li",
        "lia", "lian", "liang", "liao", "lie", "lin", "ling", "liu", "long", "lo", "lou", "lu",
        "luan", "lue", "lun", "luo", "lv", "ma", "mai", "man", "mang", "mao", "me", "mei", "men",
        "meng", "mi", "mian", "miao", "mie", "min", "ming", "miu", "mo", "mou", "mu", "na", "nai",
        "nan", "nang", "nao", "ne", "nei", "nen", "neng", "ni", "nian", "niang", "niao", "nie",
        "nin", "ning", "niu", "nong", "nou", "nu", "nuan", "nue", "nun", "nuo", "nv", "o", "ou",
        "pa", "pai", "pan", "pang", "pao", "pei", "pen", "peng", "pi", "pian", "piao", "pie",
        "pin", "ping", "po", "pou", "pu", "qi", "qia", "qian", "qiang", "qie", "qin", "qing",
        "qiong", "qiu", "qu", "quan", "que", "qun", "qv", "ran", "rang", "rao", "re", "ren",
        "reng", "ri", "rong", "rou", "ru", "ruan", "rui", "run", "ruo", "sa", "sai", "san", "sang",
        "sao", "se", "sen", "seng", "sha", "shai", "shan", "shang", "shao", "she", "shei", "shen",
        "sheng", "shi", "shou", "shu", "shua", "shuai", "shuan", "shuang", "shui", "shun", "shuo",
        "si", "song", "sou", "su", "suan", "sui", "sun", "suo", "ta", "tai", "tan", "tang", "tao",
        "te", "tei", "teng", "ti", "tian", "tiao", "tie", "ting", "tong", "tou", "tu", "tuan",
        "tui", "tun", "tuo", "wa", "wai", "wan", "wang", "wei", "wen", "weng", "wo", "wu", "xi",
        "xia", "xian", "xiang", "xiao", "xie", "xin", "xing", "xiong", "xiu", "xu", "xuan", "xue",
        "xun", "xv", "ya", "yan", "yang", "yao", "ye", "yi", "yin", "ying", "yo", "yong", "you",
        "yu", "yuan", "yue", "yun", "za", "zai", "zan", "zang", "zao", "ze", "zei", "zen", "zeng",
        "zha", "zhai", "zhan", "zhang", "zhao", "zhe", "zhen", "zheng", "zhi", "zhong", "zhou",
        "zhu", "zhua", "zhuai", "zhuan", "zhuang", "zhui", "zhun", "zhuo", "zi", "zong", "zou",
        "zu", "zuan", "zui", "zun", "zuo",
    ]
    .into_iter()
    .collect()
});

/// 将一个全部由有效拼音拼接成的字符串进行拆分
///
/// 如 jinan => ["ji nan", "jin an"]；zhang => ["zhang"]；zhangssan => []
///
/// 算法来自 https://github.com/xmflswood/pinyin-match/blob/master/src/core.js#L57
pub fn pinyin_split(value: impl AsRef<str>) -> Vec<String> {
    let pinyin = value.as_ref();
    // 去除声调
    let pinyin = to_plain(pinyin);
    // 用来保存所有符合要求的切分之后的拼音
    let mut all_pinyin_slice = Vec::<String>::new();
    // 记录从某个位置开始之后的子串存在有效的拼音分割方案
    let mut possible = vec![true; pinyin.len() + 1];
    // 每一次切分的方案
    let mut solution = Vec::new();
    get_all_pinyin_slice(
        0,
        &pinyin,
        &mut solution,
        &mut all_pinyin_slice,
        &VALID_PINYIN,
        &mut possible,
    );
    all_pinyin_slice
}

fn to_plain(pinyin: &str) -> String {
    pinyin
        .chars()
        .map(|ch| {
            if let Some(char) = TONE_TO_PLAIN.get(&ch) {
                char.to_owned()
            } else {
                ch
            }
        })
        .collect::<String>()
}

fn get_all_pinyin_slice(
    start: usize,
    pinyin: &str,
    solution: &mut Vec<String>,
    all_pinyin_slice: &mut Vec<String>,
    valid_pinyin_segments: &HashSet<&str>,
    possible: &mut [bool],
) {
    if start == pinyin.len() {
        all_pinyin_slice.push(solution.join(" "));
        return;
    }

    for i in start..pinyin.len() {
        let slice = &pinyin[start..=i];
        if valid_pinyin_segments.contains(slice) && possible[i + 1] {
            solution.push(slice.to_string());
            // 记录当前的切片的数量
            let before_change = all_pinyin_slice.len();
            get_all_pinyin_slice(
                i + 1,
                pinyin,
                solution,
                all_pinyin_slice,
                valid_pinyin_segments,
                possible,
            );
            // 没有找到新的拼音切片，将位置改为 false，用于剪枝
            if all_pinyin_slice.len() == before_change {
                possible[i + 1] = false;
            }
            // 回溯到上一次结果
            solution.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pinyin_break() {
        let pinyin = "zhong";
        let split = super::pinyin_split(pinyin);
        assert_eq!(split, ["zhong"]);
        let pinyin = "jinan";
        let split = super::pinyin_split(pinyin);
        assert_eq!(split, ["ji nan", "jin an"]);
        let pinyin = "jínan";
        let split = super::pinyin_split(pinyin);
        assert_eq!(split, ["ji nan", "jin an"]);
        let pinyin = "zhangssan";
        let split = super::pinyin_split(pinyin);
        assert!(split.is_empty())
    }
}
