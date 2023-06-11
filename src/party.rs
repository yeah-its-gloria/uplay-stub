// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use log::debug;

use crate::overlapped::OverlappedHandle;

#[no_mangle]
extern "C" fn UPLAY_PARTY_Init(unknown: u32) -> i32 {
    debug!("party init {}", unknown);
    return 1;
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_InvitePartyToGame(_overlapped: *mut OverlappedHandle) -> bool {
    debug!("party inviting game");
    return false;
}
