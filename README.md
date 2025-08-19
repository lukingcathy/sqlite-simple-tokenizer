# sqlite-plugin-demo

![License](https://img.shields.io/crates/l/PROJECT.svg)

> 这是一个使用 `rusqlite` 构建 SQLite fts5 插件的项目，其主要功能是为 SQLite 提供中文和拼音分词和检索。这个项目可以作为 Rust 的 crate 使用，也可以将其编译成动态库，在 SQLite 中加载和使用。

这个项目提供两种 SQLite 分词器，分别是 `simple_tokenizer` 和 `jieba_tokenizer`。这两种分词器均可处理汉语和英语两种语言，内置了汉语和英语常见的停词表，英语单词则进行了词根提取。

- `simple_tokenizer` 对于汉语的处理，是将单字转换成 pinyin，并且辅以 `simple_query` 函数进行匹配查询。此分词器只在文档写入的时候生效，查询应该使用 `simple_query`。该分词器对拼音的处理方式，极大程度上参考了 [simple](https://github.com/wangfenjin/simple) 这个项目，对此十分感谢 `simple` 项目提供的思路。
- `jieba_tokenizer` 对于汉语的处理，是根据 `jieba.rs` 这个库来进行。该分词器在文档查询和文档写入的时候均生效，可以正常使用 `match` 语法进行匹配。

## 在 Rust 使用这个库

在 Rust 中使用这个分词器，需要引入 `rusqlite` 依赖

```rust
let conn = Connection::open_in_memory().unwrap();
load( & conn).unwrap();
// 创建一个测试表
conn.execute("CREATE VIRTUAL TABLE t1 USING fts5(text, tokenize = 'simple');", [], ).unwrap();
// 插入数据
conn.execute(r#"INSERT INTO t1(text) VALUES ('中华人民共和国国歌'),('静夜思'),('国家'),('举头望明月'),('like'),('liking'),('liked'),('I''m making a sqlite tokenizer'),('I''m learning English');"#, [], ).unwrap();
// 查询
let mut stmt = conn.prepare("SELECT * FROM t1 WHERE text MATCH simple_query('国');").unwrap();
// 结果处理
let result = stmt.query_map([], | row| Ok(row.get::<_, String>(0).unwrap())).unwrap();
let mut vec = Vec::new();
for row in result {
let row = row.unwrap();
vec.push(row)
}
assert_eq!(["中华人民共和国国歌", "国家"], vec.as_slice());
```

## 许可

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### 贡献

除非您另有明确说明，否则任何您提交的代码许可应按上述 Apache 和 MIT 双重许可，并没有任何附加条款或条件。