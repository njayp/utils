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
        };
    }
}

mod user_input_list;