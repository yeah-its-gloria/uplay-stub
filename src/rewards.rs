// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::c_void;

#[no_mangle]
extern "C" fn UPLAY_WIN_ReleaseRewardList(_unknown: *mut c_void) -> bool {
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_WIN_RefreshActions() -> bool {
    return false;
}