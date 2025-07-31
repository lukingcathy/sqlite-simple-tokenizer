use rusqlite::Connection;
use rusqlite::ffi::{sqlite3, sqlite3_api_routines};
use std::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_sqlitesimpletokenizer_init(
    db: *mut sqlite3,
    pz_err_msg: *mut *mut c_char,
    p_api: *mut sqlite3_api_routines,
) -> c_int {
    unsafe { Connection::extension_init2(db, pz_err_msg, p_api, init) }
}

fn init(db: Connection) -> rusqlite::Result<bool> {
    todo!()
}
