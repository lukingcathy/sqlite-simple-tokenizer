## 0.3.0

发布于 2025-08-24

### 改变

* 不再使用 `Box<dyn std::error::Error>` 来捕获错误，提供一个明确的静态的错误类型

----

## 0.2.1

发布于 2025-08-24

### 修复

* 修复在 Windows 上编译和测试失败的问题，使得 CI 正常

### 维护

* `build.rs` 不再依赖 `phf`

----

## 0.2.0

发布于 2025-08-22

### 改变

* 调整 `simple_tokenizer` 的行为，允许将单字转换成 pinyin 的处理，在任何阶段的分词上都起效

### 修复

* 修复 `simple_tokenizer` 在启用 pinyin 时，设置停用停词表不生效的问题

* 修复 `simple_query` 在拆分字符串成拼音子串时，获取字符个数不正确的问题

### 维护

* 不再依赖 `anyhow`

----

## 0.1.0

发布于 2025-08-20

### 新增

* 完成 `simple_tokenizer` 和 `jieba_tokenizer` 功能

* 为 `simple_tokenizer` 提供查询辅助函数 `simple_query`