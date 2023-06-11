// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::c_void;

use log::info;

use crate::overlapped::OverlappedHandle;

type SaveHandle = u32;

#[no_mangle]
extern "C" fn UPLAY_SAVE_Close(_handle: SaveHandle) -> bool {
    info!("SAVE CLOSE");
    return true;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_GetSavegames(
    _unknown: *mut *mut u32,
    _overlapped: *mut OverlappedHandle,
) -> i32 {
    unsafe { **_unknown = 5; }
    info!("GET SAVES");
    return 1;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_Open(
    _unknown1: i32,
    _unknown2: i32,
    _unknown3: *mut *mut i32,
    _unknown4: *mut i32,
) -> i32 {
    info!("SAVE OPEN");

    unsafe { *_unknown4 = 5; }
    return 1;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_Read(
    _unknown1: i32,
    _unknown2: *mut c_void,
    _unknown3: i32,
    _unknown4: *mut *mut c_void,
    _unknown5: *mut u32,
    _unknown6: *mut i32,
) -> i32 {
    info!("SAVE READ");
    return 0;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_ReleaseGameList(_unknown: *mut c_void) -> bool {
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_Remove(_unknown: i32, _overlapped: *mut OverlappedHandle) -> bool {
    info!("SAVE REMOVE");
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_SetName(_handle: SaveHandle, _unknown: *mut *mut i32) -> i32 {
    info!("SAVE NAME");
    return 0;
}

#[no_mangle]
extern "C" fn UPLAY_SAVE_Write(
    _unknown1: i32,
    _unknown2: *const c_void,
    _unknown3: *const *const c_void,
    _unknown4: *mut i32,
) -> i32 {
    info!("SAVE WRITE");
    return 0;
}
