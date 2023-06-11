// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::c_void;

use crate::overlapped::OverlappedHandle;

#[no_mangle]
extern "C" fn UPLAY_PRODUCT_GetProductList(
    _unknown1: i32,
    _unknown2: *mut c_void,
    _overlapped: OverlappedHandle,
) -> bool {
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_PRODUCT_ReleaseProductList(_unknown: *mut c_void) -> i32 {
    return 1;
}
