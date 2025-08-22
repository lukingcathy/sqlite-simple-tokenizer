## 0.2.0

发布于 2025-08-22.

### 改变

* 调整 `simple_tokenizer` 的行为，允许将单字转换成 pinyin 的处理，在任何阶段的分词上都起效

### 修复

* 修复 `simple_tokenizer` 在启用 pinyin 时，设置停用停词表不生效的问题

* 修复 `simple_query` 在拆分字符串成拼音子串时，获取字符个数不正确的问题

### 维护

* 不再依赖 `anyhow`

----

## 0.1.0

发布于 2025-08-20.

### 新增

* 完成 `simple_tokenizer` 和 `jieba_tokenizer` 功能

* 为 `simple_tokenizer` 提供查询辅助函数 `simple_query`