//#![allow(unused_imports)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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

mod win_app_search;