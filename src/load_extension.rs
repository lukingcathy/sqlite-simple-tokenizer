use crate::tokenizer::jieba_tokenizer::JiebaTokenizer;
use crate::tokenizer::register_tokenizer;
use crate::tokenizer::simple_tokenizer::SimpleTokenizer;
use rusqlite::Connection;

pub fn create_scalar_functions(connection: &Connection) -> anyhow::Result<()> {
    Ok(())
}

pub fn load_fts5_extension(connection: &Connection) -> anyhow::Result<()> {
    // 注册 simple_tokenizer
    register_tokenizer::<SimpleTokenizer>(connection, ())?;
    // 注册 jieba_tokenizer
    register_tokenizer::<JiebaTokenizer>(connection, ())?;
    Ok(())
}
