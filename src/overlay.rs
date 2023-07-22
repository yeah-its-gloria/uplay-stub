// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::c_void;

use log::debug;

use crate::overlapped::OverlappedHandle;

#[no_mangle]
extern "C" fn UPLAY_OVERLAY_Show(_unknown: i32, _overlapped: *mut OverlappedHandle) -> bool {
    debug!("overlay show");
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_OVERLAY_SetShopUrl(_unknown: *mut *mut c_void, _overlapped: *mut OverlappedHandle) -> bool {
    debug!("overlay SetShopUrl");
    return false;
}