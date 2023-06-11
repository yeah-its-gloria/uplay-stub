// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use log::{debug, info, LevelFilter};

use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use lazy_static::lazy_static;

use std::{
    ffi::{c_char, c_void, CStr},
    ptr, sync::Mutex,
};

use crate::{
    config::{load_config, Configuration},
    overlapped::OverlappedHandle,
};

#[allow(dead_code)]
#[repr(i32)]
enum StartupResult {
    Success = 0,
    Failure = 1,
    CannotFindUplay = 3,
}

lazy_static! {
    pub(crate) static ref CONFIG: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[no_mangle]
extern "C" fn UPLAY_Start(uplay_id: u32, _flags: u32) -> StartupResult {
    return UPLAY_Startup(uplay_id, 0, ptr::null());
}

#[no_mangle]
extern "C" fn UPLAY_Startup(
    uplay_id: u32,
    game_version: u32,
    language_id: *const c_char,
) -> StartupResult {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("uplay-stub.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    match load_config() {
        Ok(config) => *CONFIG.lock().unwrap() = Some(config),
        Err(_) => {
            panic!("configuration not found"); // TODO:  show the user a message box instead
        }
    };

    let language_id_str = unsafe { CStr::from_ptr(language_id).to_str().unwrap() };
    info!(
        "STARTUP game {} ver {} LID {}",
        uplay_id, game_version, language_id_str
    );

    return StartupResult::Success;
}

#[no_mangle]
extern "C" fn UPLAY_Quit(unknown: u32) {
    info!("quitting {}", unknown);
}

#[no_mangle]
extern "C" fn UPLAY_Release(_unknown: *mut c_void) -> i32 {
    debug!("releasing");
    return 1;
}

#[no_mangle]
extern "C" fn UPLAY_Update() -> bool {
    return true;
}

#[no_mangle]
extern "C" fn UPLAY_PeekNextEvent() -> i32 {
    debug!("peeking events");
    return 0;
}

#[no_mangle]
extern "C" fn UPLAY_GetNextEvent(_overlapped: *mut OverlappedHandle) -> bool {
    debug!("fetching next event");
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_SetLanguage(_unknown: *mut i32) -> bool {
    info!("LANGUAGE");
    return true;
}

#[no_mangle]
extern "C" fn UPLAY_GetLastError(_message: *const u16) -> i32 {
    // piped to a call to MessageBoxW, 0 indicates we gave an error message back

    return 1;
}
