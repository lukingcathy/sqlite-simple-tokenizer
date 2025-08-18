#[cfg(feature = "build_extension")]
mod create_extension;
mod load_extension;
mod pinyin;
mod tokenizer;
mod utils;

include!(concat!(env!("OUT_DIR"), "/stopword_data.rs"));

use load_extension::create_scalar_functions;
use load_extension::load_fts5_extension;
use log::LevelFilter;
use rusqlite::Connection;
use utils::init_logging;

pub fn load(connection: &Connection) -> anyhow::Result<()> {
    load_with_loglevel(connection, LevelFilter::Info)
}

pub fn load_with_loglevel(connection: &Connection, log_level: LevelFilter) -> anyhow::Result<()> {
    init_logging(log_level);
    // 加载拓展函数
    create_scalar_functions(connection)?;
    // 加载 fts5 拓展
    load_fts5_extension(connection)
}
