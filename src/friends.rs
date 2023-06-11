// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use log::info;

use crate::overlapped::OverlappedHandle;

#[no_mangle]
extern "C" fn UPLAY_FRIENDS_GetFriendList(
    _unknown1: u32,
    _overlapped: *mut OverlappedHandle,
) -> bool {
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_FRIENDS_Init(unknown: u32) {
    info!("FRIENDS init {}", unknown);
}

#[no_mangle]
extern "C" fn UPLAY_FRIENDS_InviteToGame(
    _unknown1: *mut u32,
    _overlapped: *mut OverlappedHandle,
) -> bool {
    return false;
}
