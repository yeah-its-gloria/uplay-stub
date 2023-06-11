// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::ffi::{c_char, CString};

pub(crate) fn copy_str_to_c_char(input: &str, output: *mut c_char) {
    let string = CString::new(input).unwrap();

    unsafe {
        string.as_ptr().copy_to(output, input.len());
    }
}
