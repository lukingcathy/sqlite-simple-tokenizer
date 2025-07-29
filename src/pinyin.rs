use phf::{phf_map, phf_set};
use std::char;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// 通过字符获取拼音
pub fn get_pinyin(ch: &char) -> Option<&[String]> {
    let Some(pinyin) = PINYIN_DIRT.get(&ch) else {
        return None;
    };
    Some(&pinyin[..])
}

/// 分割拼音串
/// 如果提供空串、一个字符的拼音串、超过 20 个字符的拼音串均不处理，原样返回
///
/// 例如
/// - `ba` 得到 {"ba", "b+a"}
/// - "zhangliangying` 得到  `{"zhangliangying", "zhang+li+ang+yin+g", "zhang+li+ang+ying", "zhang+liang+yin+g", "zhang+liang+ying", "z+h+a+n+g+l+i+a+n+g+y+i+n+g"}`
/// - `zhangliangy` 得到  `{"zhangliangy", "zhang+li+ang+y", "zhang+liang+y", "z+h+a+n+g+l+i+a+n+g+y+i+n+g"}`
pub fn split_pinyin(input: &str) -> HashSet<String> {
    let len = input.len();
    const MAX_LEN: usize = 20;
    if len <= 1 || len > MAX_LEN {
        return HashSet::from([input.to_owned()]);
    }
    let spaced = input
        .chars()
        .fold(String::new(), |mut acc, ch| {
            acc.push('+');
            acc.push(ch);
            acc
        })
        .chars()
        .skip(1)
        .collect::<String>();
    if len > 2 {
        let pinyin = split_pinyin_with_index(input, 0, len);
        // 去重
        let mut pinyin = pinyin.into_iter().collect::<HashSet<_>>();
        pinyin.insert(spaced);
        pinyin.insert(input.to_owned());
        pinyin
    } else {
        HashSet::from([input.to_owned(), spaced])
    }
}

/// 分割拼音
fn split_pinyin_with_index(input: &str, begin: usize, end: usize) -> Vec<String> {
    if begin >= end {
        return Vec::new();
    }
    if begin == end - 1 {
        return vec![input[begin..end].to_owned()];
    }
    let mut result = Vec::<String>::new();
    let full_str = &input[begin..end];
    if PINYIN_PREFIX.contains(full_str) || PINYIN_VALID.contains(full_str) {
        result.push(full_str.to_owned());
    }
    let mut start = begin + 1;
    while start < end {
        let first = &input[begin..start];
        if !PINYIN_VALID.contains(first) {
            start += 1;
            continue;
        }
        let tmp = split_pinyin_with_index(input, start, end);
        for s in tmp {
            result.push(format!("{first}+{s}"));
        }
        start += 1
    }
    result
}

/// 汉字码点和拼音的映射表
static DEFAULT_PINYIN_DATA: &'static str = include_str!("data/pinyin.txt");

/// 借助汉字码点和拼音的映射表，在运行时构建一个 char 与拼音映射的全局字典
static PINYIN_DIRT: LazyLock<HashMap<char, Vec<String>>> = LazyLock::new(|| {
    let mut dirt = HashMap::<char, Vec<String>>::new();
    for line in DEFAULT_PINYIN_DATA.split("\n").into_iter() {
        if line.is_empty() || line.starts_with("#") {
            continue;
        }
        // 第一个是码点，第二个是拼音集合
        let mut codepoint_and_pinyin = line.split(": ");
        let codepoint = if let Some(codepoint) = codepoint_and_pinyin.next() {
            char::from_u32(u32::from_str_radix(&codepoint[2..], 16).unwrap()).unwrap()
        } else {
            char::default()
        };
        let pinyin_vec = if let Some(pinyin) = codepoint_and_pinyin.next() {
            to_plain(pinyin)
        } else {
            Vec::default()
        };
        dirt.insert(codepoint, pinyin_vec);
    }
    dirt
});

/// 将拼音中带有声调的韵母转换为不带声调的韵母
fn to_plain(input: &str) -> Vec<String> {
    let mut pinyin_set = HashSet::new();
    let mut value = String::new();
    for char in input.chars() {
        if char == ',' {
            let str = value.clone();
            pinyin_set.insert(str);
            value.clear();
            continue;
        }
        if let Some(ch) = TONE_TO_PLAIN.get(&char) {
            value.push(*ch)
        } else {
            value.push(char)
        }
    }
    pinyin_set.into_iter().collect()
}

/// 带声调的韵母和和不带声调的韵母的映射
static TONE_TO_PLAIN: phf::Map<char, char> = phf_map! {
    'ā'=>'a', 'á'=>'a', 'ǎ'=>'a', 'à'=>'a',
    'ē'=>'e', 'é'=>'e', 'ě'=>'e', 'è'=>'e', 'ế'=>'e', 'ề'=>'e', 'ê'=>'e',
    'ō'=>'o', 'ó'=>'o', 'ǒ'=>'o', 'ò'=>'o',
    'ī'=>'i', 'í'=>'i', 'ǐ'=>'i', 'ì'=>'i',
    'ū'=>'u', 'ú'=>'u', 'ǔ'=>'u', 'ù'=>'u',
    'ǘ'=>'u', 'ǚ'=>'u', 'ǜ'=>'u', 'ü'=>'u',
    'ń'=>'n', 'ň'=>'n', 'ǹ'=>'n',
    'ḿ'=>'m',
};

/// 不是合法拼音，但可以是前缀，只能出现在结尾
static PINYIN_PREFIX: phf::Set<&'static str> = phf_set! {
    "be","bia",
    "ch","cho","chon","chua","co","con","cua",
    "din","don","do","dua",
    "fe",
    "go","gon",
    "ho","hon",
    "len","lon","lua",
    "mia",
    "nia","no","non","nua",
    "pe","pia",
    "qio","qion","qua",
    "ra","ro","ron","rua",
    "sh","sho","so","son","sua",
    "ten","tia","tin","to","ton","tua",
    "we",
    "xio","xion","xua",
    "yon","yua",
    "zh","zho","zhon","zo","zon","zua",
};

/// 合法拼音
static PINYIN_VALID: phf::Set<&'static str> = phf_set! {
    "a", "ai", "an", "ang", "ao",
    "ba", "bai", "ban", "bang", "bao", "bei", "ben", "beng", "bi", "bian", "biao", "bie", "bin", "bing", "bo", "bu",
    "ca", "cai", "can", "cang", "cao", "ce", "cen", "ceng", "cha", "chai", "chan", "chang", "chao", "che", "chen", "cheng", "chi", "chong", "chou", "chu", "chuai", "chuan", "chuang", "chui", "chun", "chuo", "ci", "cong", "cou", "cu", "cuan", "cui", "cun", "cuo",
    "da", "dai", "dan", "dang", "dao", "de", "dei", "den", "deng", "di", "dia", "dian", "diao", "die", "ding", "diu", "dong", "dou", "du", "duan", "dui", "dun", "duo",
    "e", "ei", "en", "eng", "er",
    "fa", "fan", "fang", "fei", "fen", "feng", "fo", "fou", "fu",
    "ga", "gai", "gan", "gang", "gao", "ge", "gei", "gen", "geng", "gong", "gou", "gu", "gua", "guai", "guan", "guang", "gui", "gun", "guo",
    "ha", "hai", "han", "hang", "hao", "he", "hei", "hen", "heng", "hong", "hou", "hu", "hua", "huai", "huan", "huang", "hui", "hun", "huo",
    // "i"=>[],
    "ji", "jia", "jian", "jiang", "qiao", "jiao", "jie", "jin", "jing", "jiong", "jiu", "ju", "juan", "jue", "jun","jv",
    "ka", "kai", "kan", "kang", "kao", "ke", "kei", "ken", "keng", "kong", "kou", "ku", "kua", "kuai", "kuan", "kuang", "kui", "kun", "kuo",
    "la", "lai", "lan", "lang", "lao", "le", "lei", "leng", "li", "lia", "lian", "liang", "liao", "lie", "lin", "ling", "liu", "long", "lo", "lou", "lu", "luan", "lue", "lun", "luo","lv",
    "ma", "mai", "man", "mang", "mao", "me", "mei", "men", "meng", "mi", "mian", "miao", "mie", "min", "ming", "miu", "mo", "mou", "mu",
    "na", "nai", "nan", "nang", "nao", "ne", "nei", "nen", "neng", "ni", "nian", "niang", "niao", "nie", "nin", "ning", "niu", "nong", "nou", "nu", "nuan", "nue", "nun", "nuo", "nv",
    "o", "ou",
    "pa", "pai", "pan", "pang", "pao", "pei", "pen",
    "peng", "pi", "pian", "piao", "pie", "pin", "ping", "po", "pou", "pu",
    "qi", "qia", "qian", "qiang", "qie", "qin", "qing", "qiong", "qiu", "qu", "quan", "que", "qun","qv",
    "ran", "rang", "rao", "re", "ren", "reng", "ri", "rong", "rou", "ru", "ruan", "rui", "run", "ruo",
    "sa", "sai", "san", "sang", "sao", "se", "sen", "seng", "sha", "shai", "shan", "shang", "shao", "she", "shei", "shen", "sheng", "shi", "shou", "shu", "shua", "shuai", "shuan", "shuang", "shui", "shun", "shuo", "si", "song", "sou", "su", "suan", "sui", "sun", "suo",
    "ta", "tai", "tan", "tang", "tao", "te", "tei", "teng", "ti", "tian", "tiao", "tie", "ting", "tong", "tou", "tu", "tuan", "tui", "tun", "tuo",
    // "u"=>[],
    // "v"=>[],
    "wa", "wai", "wan", "wang", "wei", "wen", "weng", "wo", "wu",
    "xi", "xia", "xian", "xiang", "xiao", "xie", "xin", "xing", "xiong", "xiu", "xu", "xuan", "xue", "xun","xv",
    "ya", "yan", "yang","yao", "ye", "yi", "yin", "ying", "yo", "yong", "you", "yu", "yuan", "yue", "yun",
    "za", "zai", "zan", "zang", "zao", "ze", "zei", "zen", "zeng", "zha", "zhai", "zhan", "zhang", "zhao", "zhe", "zhen", "zheng", "zhi", "zhong", "zhou", "zhu", "zhua", "zhuai", "zhuan", "zhuang", "zhui", "zhun", "zhuo", "zi", "zong", "zou", "zu", "zuan", "zui", "zun", "zuo",
};

#[cfg(test)]
mod tests {
    use crate::pinyin::{PINYIN_DIRT, get_pinyin, split_pinyin};
    use std::collections::HashSet;

    #[test]
    fn test_pinyin_dirt() {
        let ch = '中';
        let pinyin = PINYIN_DIRT.get(&ch);
        assert_eq!(Some(&vec!["zhong".to_owned()]), pinyin);
        let ch = '说';
        let pinyin = PINYIN_DIRT.get(&ch);
        assert_eq!(Some(&vec!["shuo".to_owned(), "shui".to_owned()]), pinyin);
    }

    #[test]
    fn test_get_pinyin() {
        let ch = '中';
        let pinyin = get_pinyin(&ch);
        assert_eq!(Some(&vec!["zhong".to_owned()][..]), pinyin);
        let ch = '说';
        let pinyin = get_pinyin(&ch);
        assert_eq!(
            Some(&vec!["shuo".to_owned(), "shui".to_owned()][..]),
            pinyin
        );
    }

    #[test]
    fn test_split_pinyin() {
        let input = "";
        assert_eq!(HashSet::from(["".to_owned()]), split_pinyin(input));
        let input = "a";
        assert_eq!(HashSet::from(["a".to_owned()]), split_pinyin(input));
        let input = "ba";
        assert_eq!(
            HashSet::from(["ba".to_owned(), "b+a".to_owned()]),
            split_pinyin(input)
        );
        let input = "zhangliangy";
        assert_eq!(
            HashSet::from([
                "zhangliangy".to_owned(),
                "zhang+li+ang+y".to_owned(),
                "zhang+liang+y".to_owned(),
                "z+h+a+n+g+l+i+a+n+g+y".to_owned()
            ]),
            split_pinyin(input)
        );
        let input = "zhangliangying";
        assert_eq!(
            HashSet::from([
                "zhangliangying".to_owned(),
                "zhang+li+ang+ying".to_owned(),
                "zhang+li+ang+yin+g".to_owned(),
                "zhang+liang+ying".to_owned(),
                "zhang+liang+yin+g".to_owned(),
                "z+h+a+n+g+l+i+a+n+g+y+i+n+g".to_owned()
            ]),
            split_pinyin(input)
        );
        let input = "zhangliangyingzhangliangying";
        assert_eq!(
            HashSet::from(["zhangliangyingzhangliangying".to_owned()]),
            split_pinyin(input)
        );
    }
}
