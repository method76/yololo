extern crate serde_json; // 1.0.59

use std::{env, borrow::Cow, sync::mpsc, thread, path::Path, fs, io::Write, str, ffi::OsString, ffi::OsStr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub labeled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextFile {
    pub url: String,
    pub labeled: bool,
}
