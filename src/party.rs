// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::c_void;

use log::debug;

use crate::overlapped::OverlappedHandle;

#[no_mangle]
extern "C" fn UPLAY_PARTY_Init(unknown: u32) -> i32 {
    debug!("party init {}", unknown);
    return 1;
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_InvitePartyToGame(_overlapped: *mut OverlappedHandle) -> bool {
    debug!("InvitePartyToGame");
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_InviteToParty(_unknown: *mut c_void, _overlapped: *mut OverlappedHandle) {
    debug!("InviteToParty");

    // supposedly uses strings to identify users, maybe _unknown is a const char**?
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_GetFullMemberList(_unknown: *mut c_void) -> bool {
    debug!("GetFullMemberList");

    return false;
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_IsPartyLeader(_unknown: *mut c_void) -> bool {
    debug!("IsPartyLeader");
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_SetGuest(
    _unknown: *mut c_void,
    _overlapped: *mut OverlappedHandle,
) -> bool {
    debug!("SetGuest");
    return false;
}

#[no_mangle]
extern "C" fn UPLAY_PARTY_SetUserData(
    _unknown: *mut c_void
) -> bool {
    // first byte sets length? awkward
    // cannot be larger than 1024 bytes

    debug!("SetUserData");
    return false;
}


#[no_mangle]
extern "C" fn UPLAY_PARTY_ShowGameInviteOverlayUI(_id: u32) -> bool {
    debug!("ShowGameInviteOverlayUI");
    return false;
}
