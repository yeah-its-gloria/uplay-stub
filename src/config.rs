// SPDX-FileCopyrightText: Copyright 2023 Gloria G.
// SPDX-License-Identifier: BSD-2-Clause

use std::{fs, io};

use serde::Deserialize;

pub(crate) enum ConfigurationError {
    FSError(io::Error),
    ParseError(serde_json::Error),
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Configuration {
    pub username: String,
    pub password: String,
    pub id: String
}

pub(crate) fn load_config() -> Result<Configuration, ConfigurationError> {
    let file: fs::File;
    match fs::File::open("config.json") {
        Ok(handle) => file = handle,
        Err(fail) => {
            return Err(ConfigurationError::FSError(fail));
        }
    }

    match serde_json::from_reader::<fs::File, Configuration>(file) {
        Ok(config) => {
            return Ok(config);
        }

        Err(fail) => {
            return Err(ConfigurationError::ParseError(fail));
        }
    }
}
