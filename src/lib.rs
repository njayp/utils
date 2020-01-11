#![allow(unused_imports)]

#[macro_use]
mod macros {
    macro_rules! imp {
        () => {
            
            use std::error::Error;
            use std::io;
            use std::path::PathBuf;
            use std::process::Command;

            //use lazy_static::lazy_static;
            use regex::Regex;
            //use serde::{Deserialize, Serialize};
            //use soup::*;
        };
    }
}

pub use self::win_app_search::find_windows_app;
mod win_app_search;

pub use self::user_input_list::user_select_from_list;
mod user_input_list;
