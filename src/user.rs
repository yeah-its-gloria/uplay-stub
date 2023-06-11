// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::{c_char, c_void};

use log::info;

use crate::{overlapped::OverlappedHandle, utilities::copy_str_to_c_char, core::CONFIG};

#[repr(i32)]
#[allow(dead_code)]
enum UserResult {
    Failure = 0,
    Success = 1,
}

#[no_mangle]
extern "C" fn UPLAY_USER_ClearGameSession() -> i32 {
    return 0;
}

#[no_mangle]
extern "C" fn UPLAY_USER_GetAccountId(output: *mut c_char) -> UserResult {
    copy_str_to_c_char(&CONFIG.lock().unwrap().clone().unwrap().id, output); // max length is 64
    return UserResult::Success;
}

#[no_mangle]
extern "C" fn UPLAY_USER_GetAccountIdUtf8() {} // FC3 doesn't actually get anything from these functions, do they even have any purpose?

#[no_mangle]
extern "C" fn UPLAY_USER_GetCdKeys(
    _unknown: *mut *mut i32,
    _overlapped: *mut OverlappedHandle,
) -> UserResult {
    return UserResult::Failure; // TODO: result behavior seems to be irregular from the other USER methods
}

#[no_mangle]
extern "C" fn UPLAY_USER_GetPassword(output: *mut c_char) -> UserResult {
    copy_str_to_c_char(&CONFIG.lock().unwrap().clone().unwrap().password, output); // max length is 64
    return UserResult::Success;
}

#[no_mangle]
extern "C" fn UPLAY_USER_GetUsername(output: *mut c_char, _unknown: u32) -> UserResult {
    copy_str_to_c_char(&CONFIG.lock().unwrap().clone().unwrap().username, output); // max length is 256
    return UserResult::Success;
}

#[no_mangle]
extern "C" fn UPLAY_USER_GetNameUtf8() {} // you saw the note for UPLAY_USER_GetAccountIdUtf8

#[no_mangle]
extern "C" fn UPLAY_USER_SetGameSession(
    flags: i32,
    _unknown1: *mut i32,
    _unknown2: *mut *mut c_void,
    _unknown3: u32,
) -> UserResult {
    info!("SET USER SESSION flags {}", flags); // TODO: figure out when specifically this is called
    return UserResult::Success;
}
