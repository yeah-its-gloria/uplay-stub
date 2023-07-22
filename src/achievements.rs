// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::c_void;

use log::info;

use crate::overlapped::OverlappedHandle;

#[allow(dead_code)]
#[repr(i32)]
enum AchievementResult {
    Success,
    MustAwait,
}

#[no_mangle]
extern "C" fn UPLAY_ACH_EarnAchievement(
    achievement_id: i32,
    _overlapped: *mut OverlappedHandle,
) -> AchievementResult {
    info!("achievement earned id {}", achievement_id);
    return AchievementResult::Success;
}

#[no_mangle]
extern "C" fn UPLAY_ACH_GetAchievements(
    _unknown: u32,
    _unknown2: *mut *mut c_void,
    _unknown3: *mut c_void,
    _overlapped: *mut OverlappedHandle,
) -> bool {
    return false;
}
