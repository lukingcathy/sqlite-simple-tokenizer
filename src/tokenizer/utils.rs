use crate::pinyin::has_pinyin;
use unicode_normalization::UnicodeNormalization;

pub(super) fn need_pinyin(word: &str) -> bool {
    if word.is_empty() || word.len() > 1 {
        // 空串，或者长度大于 1 的单词，不需要 pinyin 处理
        return false;
    }
    if let Some(ch) = word.chars().next() {
        return has_pinyin(&ch);
    }
    false
}

/// 对单词做归一化，并转换成小写
/// 如果全部都是由 ascii 字符组成的单词，并且长度超过 1，需要返回一个变量用来提示后续步骤做词干提取
pub(super) fn make_lowercase(word: &str, buf: &mut String) -> bool {
    buf.clear();
    let mut need_stem = true;
    for ch in word.nfkc() {
        if is_diacritic(ch) {
            continue;
        }
        if ch.is_ascii() {
            buf.push(ch.to_ascii_lowercase());
        } else {
            need_stem = false;
            buf.extend(ch.to_lowercase());
        }
    }
    if buf.len() <= 1 {
        // 单个字符不需要提取词干
        need_stem = false;
    }
    need_stem
}

fn is_diacritic(ch: char) -> bool {
    ('\u{0300}'..='\u{036f}').contains(&ch)
}
