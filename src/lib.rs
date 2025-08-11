mod add_function;
#[cfg(feature = "build_extension")]
mod create_extension;
pub mod pinyin;
pub mod tokenizer;
mod utils;

include!(concat!(env!("OUT_DIR"), "/stopword_data.rs"));

use add_function::create_scalar_functions;
use log::LevelFilter;
use rusqlite::Connection;
use utils::init_logging;

pub fn load(connection: &Connection) -> anyhow::Result<()> {
    load_with_loglevel(connection, LevelFilter::Info)
}

pub fn load_with_loglevel(connection: &Connection, log_level: LevelFilter) -> anyhow::Result<()> {
    init_logging(log_level);
    // 加载拓展函数
    create_scalar_functions(connection)
}
