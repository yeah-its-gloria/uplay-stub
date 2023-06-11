// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

pub(crate) type OverlappedHandle = u32;

#[no_mangle]
extern "C" fn UPLAY_GetOverlappedOperationResult(
    _overlapped: OverlappedHandle,
    _result: *mut i32,
) -> bool {
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_HasOverlappedOperationCompleted(_overlapped: OverlappedHandle) -> bool {
    return false;
}
